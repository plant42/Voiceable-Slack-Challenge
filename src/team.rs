use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::teams;
use diesel::result::Error;

#[table_name = "teams"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct Team {
    pub id: Option<i32>,
    pub team_id: String,
    pub team_name: String,
    pub bot_id: String,
    pub bot_access_token: String
}

impl Team {
    pub fn create(team: Team, connection: &PgConnection) -> Team {
        diesel::insert_into(teams::table)
            .values(&team)
            .execute(connection)
            .expect("Error creating new team");

        teams::table.order(teams::id.desc()).first(connection).unwrap()
    }

    pub fn find_by_team_id(team_id: &str, connection: &PgConnection) -> Result<Team, Error> {
        teams::table.filter(teams::team_id.eq(team_id)).first(connection)
    }

    pub fn read(connection: &PgConnection) -> Vec<Team> {
        teams::table.order(teams::id).load::<Team>(connection).unwrap()
    }

    pub fn update(id: i32, team: Team, connection: &PgConnection) -> bool {
        diesel::update(teams::table.find(id)).set(&team).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(teams::table.find(id)).execute(connection).is_ok()
    }
}
