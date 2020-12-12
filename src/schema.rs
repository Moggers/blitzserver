table! {
    files (id) {
        id -> Int4,
        filename -> Varchar,
        filebinary -> Bytea,
    }
}

table! {
    games (id) {
        id -> Int4,
        name -> Varchar,
        era -> Int4,
        map_id -> Int4,
        port -> Nullable<Int4>,
    }
}

table! {
    maps (id) {
        id -> Int4,
        name -> Varchar,
        mapfile_id -> Int4,
        tgafile_id -> Int4,
        winterfile_id -> Int4,
    }
}

joinable!(games -> maps (map_id));

allow_tables_to_appear_in_same_query!(
    files,
    games,
    maps,
);
