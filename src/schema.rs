table! {
    config_layers (id) {
        id -> Unsigned<Integer>,
        configid -> Unsigned<Integer>,
        layersid -> Unsigned<Integer>,
        creation_time -> Datetime,
    }
}

table! {
    fs_layers (id) {
        id -> Unsigned<Integer>,
        digest -> Varchar,
        size -> Unsigned<Integer>,
        mediaType -> Varchar,
        path -> Text,
        creation_time -> Datetime,
    }
}

table! {
    images (id) {
        id -> Unsigned<Integer>,
        library -> Varchar,
        name -> Varchar,
        tag -> Varchar,
        fslayer_configid -> Unsigned<Integer>,
        creation_time -> Datetime,
        pull_time -> Datetime,
        push_time -> Datetime,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Char,
        name -> Char,
        password -> Char,
        admin -> Bool,
        email -> Varchar,
        comment -> Nullable<Varchar>,
    }
}

joinable!(images -> fs_layers (fslayer_configid));

allow_tables_to_appear_in_same_query!(
    config_layers,
    fs_layers,
    images,
    user,
);
