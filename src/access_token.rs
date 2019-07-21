use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::access_tokens;
use diesel::result::Error;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use db::{{PgPool, Connection}};
use authorization::{{Authorization}};

#[table_name = "access_tokens"]
#[derive(AsChangeset, Serialize, Deserialize, Queryable, Insertable)]
pub struct AccessToken {
    pub id: Option<i32>,
    pub access_token: String,
    pub scope: String,
    pub user_id: String,
    pub team_id: i32
}

impl AccessToken {
    pub fn create(token: AccessToken, connection: &PgConnection) -> AccessToken {
        diesel::insert_into(access_tokens::table)
            .values(&token)
            .execute(connection)
            .expect("Error creating new access token");

        access_tokens::table.order(access_tokens::id.desc()).first(connection).unwrap()
    }


    pub fn find_by_user_id(user_id: &str, connection: &PgConnection) -> Result<AccessToken, Error> {
        access_tokens::table.filter(access_tokens::user_id.eq(user_id)).first(connection)
    }


    pub fn find_by_id(id: i32, connection: &PgConnection) -> Result<AccessToken, Error> {
        access_tokens::table.filter(access_tokens::id.eq(id)).first(connection)
    }


    pub fn read(connection: &PgConnection) -> Vec<AccessToken> {
        access_tokens::table.order(access_tokens::id).load::<AccessToken>(connection).unwrap()
    }

    pub fn update(id: i32, token: &AccessToken, connection: &PgConnection) -> bool {
        diesel::update(access_tokens::table.find(id)).set(token).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(access_tokens::table.find(id)).execute(connection).is_ok()
    }
}


impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        //== find cookie with name
        //== find access token belonging to cookie
        //== return access token
        //== return forward if not exists
        let cookies = request.cookies();
        let cookie = cookies.get("x-voiceable-token");


        let parsed = match cookie {
            Some(t) => t.value(),
            None => return Outcome::Forward(())
        };

        println!("Cookie: {}", parsed);

        //== get pool connection
        let pool = request.guard::<State<PgPool>>()?;
        let connection = match pool.get() {
            Ok(conn) => Connection(conn),
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ()))
        };

        //== obtain authorization
        let authorization = match Authorization::find_by_token(parsed, &connection) {
            Ok(t) => t,
            Err(_) => return Outcome::Forward(())
        };

        println!("Found authorization: {}", authorization.token);

        match AccessToken::find_by_id(authorization.access_token_id, &connection) {
            Ok(t) => return Outcome::Success(t),
            Err(_) => return Outcome::Forward(())
        };

        //== extract cookie value and find authorization
    }
}
