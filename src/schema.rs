table! {
    heroes {
        id -> Nullable<Integer>,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Integer,
    }
}

table! {
    challenges {
        id -> Nullable<Integer>,
        token -> Varchar,
        challenge -> Varchar,
        type_name -> Varchar,
    }
}

table! {
    teams {
        id -> Nullable<Integer>,
        team_id -> Varchar,
        team_name -> Varchar,
        bot_id -> Varchar,
        bot_access_token -> Varchar,
    }
}


table! {
    access_tokens {
        id -> Nullable<Integer>,
        access_token -> Varchar,
        scope -> Varchar,
        user_id -> Varchar,
        team_id -> Integer,
    }
}


table! {
    authorizations {
        id -> Nullable<Integer>,
        token -> Varchar,
        access_token_id -> Integer,
    }
}
