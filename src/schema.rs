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
    }
}

table! {
    maps (id) {
        id -> Int4,
        name -> Varchar,
        mapfile_id -> Int4,
        tgafile_id -> Int4,
        winterfile_id -> Int4,
        archive_id -> Int4,
    }
}

table! {
    mods (id) {
        id -> Int4,
        dm_filename -> Varchar,
        name -> Varchar,
        file_id -> Int4,
    }
}

table! {
    nations (id) {
        id -> Int4,
        game_id -> Int4,
        nation_id -> Int4,
        name -> Varchar,
        epithet -> Varchar,
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
    }
}

joinable!(game_mods -> mods (mod_id));
joinable!(games -> maps (map_id));
joinable!(players -> files (file_id));
joinable!(turns -> files (file_id));

allow_tables_to_appear_in_same_query!(
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
