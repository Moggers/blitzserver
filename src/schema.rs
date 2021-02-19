table! {
    email_configs (id) {
        id -> Int4,
        nation_id -> Int4,
        game_id -> Int4,
        hours_before_host -> Int4,
        email_address -> Varchar,
        last_turn_notified -> Nullable<Int4>,
        subject -> Varchar,
        body -> Varchar,
        is_reminder -> Bool,
    }
}

table! {
    files (id) {
        id -> Int4,
        filename -> Varchar,
        filebinary -> Bytea,
        hash -> Int8,
    }
}

table! {
    game_mods (id) {
        id -> Int4,
        game_id -> Int4,
        mod_id -> Int4,
    }
}

table! {
    games (id) {
        id -> Int4,
        name -> Varchar,
        era -> Int4,
        map_id -> Int4,
        port -> Nullable<Int4>,
        timer -> Nullable<Int4>,
        thrones_t1 -> Int4,
        thrones_t2 -> Int4,
        thrones_t3 -> Int4,
        throne_points_required -> Int4,
        research_diff -> Int4,
        research_rand -> Bool,
        hof_size -> Int4,
        global_size -> Int4,
        indepstr -> Int4,
        magicsites -> Int4,
        eventrarity -> Int4,
        richness -> Int4,
        resources -> Int4,
        recruitment -> Int4,
        supplies -> Int4,
        startprov -> Int4,
        renaming -> Bool,
        scoregraphs -> Bool,
        nationinfo -> Bool,
        artrest -> Bool,
        teamgame -> Bool,
        clustered -> Bool,
        storyevents -> Int4,
        newailvl -> Int4,
        newai -> Bool,
        next_turn -> Nullable<Timestamp>,
        password -> Varchar,
        archived -> Bool,
    }
}

table! {
    maps (id) {
        id -> Int4,
        name -> Varchar,
        mapfile_id -> Int4,
        tgafile_id -> Int4,
        winterfile_id -> Nullable<Int4>,
        province_count -> Int4,
        uw_count -> Int4,
    }
}

table! {
    mods (id) {
        id -> Int4,
        dm_filename -> Varchar,
        name -> Varchar,
        file_id -> Int4,
        icon_file_id -> Nullable<Int4>,
    }
}

table! {
    nations (id) {
        id -> Int4,
        game_id -> Int4,
        nation_id -> Int4,
        name -> Varchar,
        epithet -> Varchar,
        filename -> Varchar,
    }
}

table! {
    player_turns (id) {
        id -> Int4,
        turn_number -> Int4,
        nation_id -> Int4,
        game_id -> Int4,
        trnfile_id -> Int4,
        twohfile_id -> Nullable<Int4>,
        archived -> Bool,
    }
}

table! {
    players (id) {
        id -> Int4,
        nationid -> Int4,
        game_id -> Int4,
        file_id -> Int4,
    }
}

table! {
    turns (id) {
        id -> Int4,
        game_id -> Int4,
        turn_number -> Int4,
        file_id -> Int4,
        created_at -> Timestamp,
        archived -> Bool,
    }
}

joinable!(email_configs -> games (game_id));
joinable!(game_mods -> mods (mod_id));
joinable!(games -> maps (map_id));
joinable!(players -> files (file_id));
joinable!(turns -> files (file_id));

allow_tables_to_appear_in_same_query!(
    email_configs,
    files,
    game_mods,
    games,
    maps,
    mods,
    nations,
    player_turns,
    players,
    turns,
);
