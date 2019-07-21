use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use schema::authorizations;

#[table_name = "authorizations"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Authorization {
    pub id: Option<i32>,
    pub token: String,
    pub access_token_id: i32
}

impl Authorization {
    pub fn create(authorization: Authorization, connection: &PgConnection) -> Authorization {
        diesel::insert_into(authorizations::table)
            .values(&authorization)
            .execute(connection)
            .expect("Error creating new authorization");

        authorizations::table.order(authorizations::id.desc()).first(connection).unwrap()
    }


    pub fn find_by_token(token: &str, connection: &PgConnection) -> Result<Authorization, Error> {
        authorizations::table.filter(authorizations::token.eq(token)).first(connection)
    }

    pub fn read(connection: &PgConnection) -> Vec<Authorization> {
        authorizations::table.order(authorizations::id).load::<Authorization>(connection).unwrap()
    }

    pub fn update(id: i32, authorization: Authorization, connection: &PgConnection) -> bool {
        diesel::update(authorizations::table.find(id)).set(&authorization).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(authorizations::table.find(id)).execute(connection).is_ok()
    }
}

