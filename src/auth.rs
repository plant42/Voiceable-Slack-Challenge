use rocket_contrib::json::{Json};
use serde_json::{{Value}};
use rocket::request::{self, FromRequest};
use rocket::http::{{RawStr, Status, Cookies, Cookie}};
use rocket::{Request, State, Outcome};
use reqwest::{Client, Response};

use rocket::response::Redirect;
use std::error::Error;
use db;
use uuid::Uuid;



use challenge::{{Challenge}};
use team::{{Team}};
use access_token::{{AccessToken}};
use authorization::{{Authorization}};


static SLACK_CLIENT_ID:&str = "156615507910.633430664976";
static SLACK_CLIENT_SECRET:&str = "5645ad43328b8150ce7896929c9a813e";


#[get("/slack?<code>")]
pub fn authorization_slack(code: Option<&RawStr>, connection: db::Connection, mut cookies: Cookies) -> Result<Redirect, Status> {

    //== check code parameter exists
    let c = match code {
        Some(code) => code.as_str(),
        _    => return Err(Status::BadRequest),
    };

    println!("Slack Authorization Code: {}", c );
    let value = slack_obtain_access_payload(c);
    let team = match &value {
        Ok(s) => slack_process_access_token_team(&s, &connection),
        _ => return Err(Status::BadRequest)
    };

    println!("Found Team Name: {} ({})", team.team_name.as_str(), team.id.unwrap());
    let user = match &value {
        Ok(s) => slack_process_access_token_user(&s, &team, &connection),
        _ => return Err(Status::BadRequest)
    };


    println!("Generating UUID Token for {}", c );

    //let authorization => slack
    let authorization = generate_cookie_token(&user, &connection);

    //== generate and set a cookie here.
    let cookie = Cookie::build("x-voiceable-token", authorization.token)
        .path("/")
        .http_only(true)
        .secure(true)
        .finish();

    cookies.add(cookie);

    Ok(Redirect::to("/"))
}

fn generate_cookie_token(accessToken: &AccessToken, connection: &db::Connection) -> Authorization {
    let token = Uuid::new_v4();

    let insert = Authorization { id: None, token: token.to_simple().to_string(), access_token_id: accessToken.team_id };
    Authorization::create(insert, &connection)
}


fn slack_obtain_access_payload(input: &str) -> Result<serde_json::Value, reqwest::Error> {

    let params = [("client_id", SLACK_CLIENT_ID), ("client_secret", SLACK_CLIENT_SECRET), ("code", input)];
    let client = reqwest::Client::new();
    let echo_json: serde_json::Value = client.get("https://slack.com/api/oauth.access")
        .query(&params)
        .send()?
        .json()?;
    println!("{:#?}", &echo_json);
    Ok(echo_json)
}



fn slack_process_access_token_user(input: &Value, team: &Team, connection: &db::Connection ) -> AccessToken {

    let user_id: &str = input["user_id"].as_str().unwrap();
    let scope: &str = input["scope"].as_str().unwrap();
    let access_token: &str = input["access_token"].as_str().unwrap();

    let accessToken = match AccessToken::find_by_user_id(&user_id, &connection) {
        Ok(user) => {
            let update = AccessToken { id: user.id, access_token: access_token.to_string(), scope: scope.to_string(), user_id: user_id.to_string(), team_id: team.id.unwrap()  };
            AccessToken::update(user.id.unwrap(), &update, connection);
            update
        },
        Err(_e) => {
            let insert = AccessToken { id: None, access_token: access_token.to_string(), scope: scope.to_string(), user_id: user_id.to_string(), team_id: team.id.unwrap()  };
            AccessToken::create(insert, &connection)
        }
    };

    accessToken

}



fn slack_process_access_token_team(input: &Value, connection: &db::Connection ) -> Team {

    let team_id: &str = input["team_id"].as_str().unwrap();
    let team_name: &str = input["team_name"].as_str().unwrap();
    let bot_id: &str = input["bot"]["bot_user_id"].as_str().unwrap();
    let bot_access_token: &str = input["bot"]["bot_access_token"].as_str().unwrap();

    let team = match Team::find_by_team_id("T4LJ3EXSS", connection) {
        Ok(team) => team,
        Err(_e) => {
            let insert = Team { id: None, team_id: team_id.to_string(), team_name : team_name.to_string(), bot_id: bot_id.to_string(), bot_access_token: bot_access_token.to_string()  };
            Team::create(insert, &connection)
        }
    };

    team

}

