use rocket_contrib::json::{Json};
use serde_json::{{Value}};

use reqwest;

use db;

use challenge::{{Challenge}};
use std::io::Read;

//header! { (XVoicemapToken, "x-voicemaps-token") => [String] }

#[post("/slack/incoming", data = "<input>")]
pub fn slack_incoming(input: String, connection: db::Connection) -> Json<Value> {

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&input).unwrap();
    let type_name: &str = v["type"].as_str().unwrap();

    println!("-{}, {}", type_name, input);

    let resp: Json<Value> = match type_name {
        "url_verification" => slack_process_challenge(&v, connection),
        _ =>  Json(Value::String("test".to_string())),
    };

    resp
}



#[get("/voiceable/incoming")]
pub fn voiceable_data() -> String {


    "test".to_string()
}


fn slack_process_challenge(input: &Value, connection: db::Connection )-> Json<Value> {

    let token: &str = input["token"].as_str().unwrap();
    let challenge: &str = input["challenge"].as_str().unwrap();
    let type_name: &str = input["type"].as_str().unwrap();

    let insert = Challenge{ id: None, token: token.to_string(), challenge: challenge.to_string(), type_name: type_name.to_string()  };
    Challenge::create(insert, &connection);

    Json(json!({
        "challenge": challenge
    }))
}
