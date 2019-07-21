use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::challenges;

#[table_name = "challenges"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Challenge {
    pub id: Option<i32>,
    pub token: String,
    pub challenge: String,
    pub type_name: String
}

impl Challenge {
    pub fn create(challenge: Challenge, connection: &PgConnection) -> Challenge {
        diesel::insert_into(challenges::table)
            .values(&challenge)
            .execute(connection)
            .expect("Error creating new challenge");

        challenges::table.order(challenges::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &PgConnection) -> Vec<Challenge> {
        challenges::table.order(challenges::id).load::<Challenge>(connection).unwrap()
    }

    pub fn update(id: i32, challenge: Challenge, connection: &PgConnection) -> bool {
        diesel::update(challenges::table.find(id)).set(&challenge).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(challenges::table.find(id)).execute(connection).is_ok()
    }
}
