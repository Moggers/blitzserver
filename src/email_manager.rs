use crate::diesel::prelude::*;
use crate::models::{PlayerTurn, EmailConfig, Game, NewEmailConfig, Turn};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use lettre::{transport::smtp::authentication::Credentials, Transport};
use std::ops::Add;

#[derive(Clone)]
pub struct EmailManager {
    pub db_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
    pub smtp_user: String,
    pub smtp_pass: String,
    pub smtp_server: String,
    pub hostname: String
}

impl EmailManager {
    pub fn update_configs(&self, game_id: i32, address: String, emails: &[NewEmailConfig]) {
        let db = self.db_pool.get().unwrap();
        db.build_transaction()
            .run::<_, diesel::result::Error, _>(|| {
                use crate::schema::email_configs::dsl as emails_dsl;
                diesel::delete(emails_dsl::email_configs)
                    .filter(
                        emails_dsl::email_address
                            .eq(address)
                            .and(emails_dsl::game_id.eq(game_id)),
                    )
                    .execute(&db)?;
                diesel::insert_into(emails_dsl::email_configs)
                    .values(emails)
                    .execute(&db)?;
                Ok(())
            })
            .unwrap();
    }

    pub fn send_notice_email<D>(
        db: &D,
        hostname: &str,
        email_config: &EmailConfig,
        mailer: &lettre::transport::smtp::SmtpTransport,
    ) where
        D: diesel::Connection<Backend = diesel::pg::Pg>,
    {
        use crate::schema::games::dsl as games_dsl;
        use crate::schema::turns::dsl as turns_dsl;

        let game: Game = games_dsl::games.filter(games_dsl::id.eq(email_config.game_id)).get_result(db).unwrap();
        let turn: Turn = turns_dsl::turns.filter(turns_dsl::game_id.eq(email_config.game_id)).order(turns_dsl::turn_number.desc()).limit(1).get_result(db).unwrap();
        let email = lettre::Message::builder()
            .from(format!("Admin <admin@{}>", hostname).parse().unwrap())
            .to(format!("<{}>", email_config.email_address).parse().unwrap())
            .subject(format!("Game {} Turn Notification", game.name))
            .body(format!("Turn {} for game {} is required within {}", turn.turn_number, game.name, game.next_turn_string()))
            .unwrap();
        mailer.send(&email).unwrap();
    }

    pub fn monitor(&self) {
        let db_pool = self.db_pool.clone();
        let mailer = lettre::transport::smtp::SmtpTransport::relay(&self.smtp_server)
            .unwrap()
            .credentials(Credentials::new(
                self.smtp_user.clone(),
                self.smtp_pass.clone(),
            ))
            .build();
        let hostname = self.hostname.clone();
        std::thread::spawn(move || loop {
            let db = db_pool.get().unwrap();
            use crate::schema::email_configs::dsl as emails_dsl;
            use crate::schema::games::dsl as games_dsl;
            use crate::schema::turns::dsl as turns_dsl;
            let email_configs: Vec<EmailConfig> = diesel::sql_query("\
SELECT ec.* FROM email_configs ec LEFT OUTER JOIN (SELECT game_id,MAX(turn_number) as turn_number FROM turns GROUP BY game_id) t ON t.game_id=ec.game_id LEFT OUTER JOIN (SELECT nation_id, game_id, MAX(turn_number) as turn_number FROM player_turns pt WHERE twohfile_id IS NOT NULL GROUP BY game_id,nation_id) pt ON pt.game_id=ec.game_id AND pt.turn_number = t.turn_number AND pt.nation_id = ec.nation_id WHERE (ec.last_turn_notified IS NULL OR t.turn_number != ec.last_turn_notified) AND (pt.turn_number IS NULL OR pt.turn_number != t.turn_number)
").load(&db).unwrap();
            for email_config in email_configs {
                let game: Game = games_dsl::games
                    .filter(games_dsl::id.eq(email_config.game_id))
                    .get_result(&db)
                    .unwrap();
                if let Some(next_turn) = game.next_turn {
                    if next_turn
                        < std::time::SystemTime::now().add(std::time::Duration::from_secs(
                            60 * 60 * email_config.hours_before_host as u64,
                        ))
                    {
                        let last_turns: Vec<Turn> = turns_dsl::turns
                            .filter(turns_dsl::game_id.eq(email_config.game_id))
                            .order(turns_dsl::turn_number.desc())
                            .limit(1)
                            .get_results(&db)
                            .unwrap();
                        let new_turn_number = match last_turns.len() {
                            0 => 0,
                            _ => last_turns[0].turn_number,
                        };
                        log::debug!(
                            "Sending turn email to {} for game {} turn {}",
                            email_config.email_address,
                            email_config.game_id,
                            new_turn_number
                        );

                        Self::send_notice_email(&db, &hostname, &email_config, &mailer);
                        diesel::update(emails_dsl::email_configs)
                            .filter(emails_dsl::id.eq(email_config.id))
                            .set(emails_dsl::last_turn_notified.eq(new_turn_number))
                            .execute(&db)
                            .unwrap();
                    }
                }
            }
            drop(db);
            std::thread::sleep(std::time::Duration::from_secs(60));
        });
    }
}
