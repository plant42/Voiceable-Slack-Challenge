// ![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]


#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate diesel;
extern crate clokwerk;
extern crate reqwest;
extern crate uuid;
use rocket::response::Redirect;

use rocket_contrib::templates::{Template};


use rocket::{routes};
use rocket_contrib::json::{Json};
use serde_json::{{Value}};
use clokwerk::{Scheduler, TimeUnits};

use clokwerk::Interval::*;
use std::thread;
use std::time::Duration;

mod db;
mod schema;

mod hero;
use hero::Hero;
use access_token::AccessToken;

mod webhooks;
mod auth;

mod challenge;
mod team;
mod access_token;
mod authorization;

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Json(Hero::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Hero::read(&connection)))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<Value> {
    let update = Hero { id: Some(id), ..hero.into_inner() };
    Json(json!({
        "success": Hero::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Hero::delete(id, &connection)
    }))
}


#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    items: Vec<&'static str>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}


#[get("/")]
pub fn index() -> Template {
    Template::render("index", &TemplateContext {
        title: "Hello",
        name:Some("Name goes here".to_string()),
        items: vec!["One", "Two", "Three"],
        parent: "layout",
    })
}

#[get("/admin")]
pub fn admin(access_token: AccessToken) -> Template {
    Template::render("admin", &TemplateContext {
        title: "Hello",
        name:Some("Name goes here".to_string()),
        items: vec!["One", "Two", "Three"],
        parent: "layout",
    })
}


#[get("/admin",  rank = 2)]
pub fn admin_redirect() -> Redirect {
    Redirect::to("/")
}


/*
#[get("/admin")]
fn admin_panel(admin: AdminUser) -> &'static str {
    "Hello, administrator. This is the admin panel!"
}

#[get("/admin", rank = 2)]
fn admin_panel_user(user: User) -> &'static str {
    "Sorry, you must be an administrator to access this page."
}

#[get("/admin", rank = 3)]
fn admin_panel_redirect() -> Redirect {
    Redirect::to("/login")
}
*/
fn main() {
    /*
    let mut scheduler = Scheduler::new();
    scheduler.every(10.seconds()).run(|| println!("Periodic task"));
    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    */
    rocket::ignite()
        .manage(db::connect())
        .mount("/", routes![index, admin])
        .mount("/hero", routes![create, update, delete])
        .mount("/heroes", routes![read])
        .mount("/webhooks", routes![webhooks::slack_incoming, webhooks::voiceable_data])
        .mount("/auth", routes![auth::authorization_slack])
        .attach(Template::custom(|_engines| {}))

        .launch();
}
