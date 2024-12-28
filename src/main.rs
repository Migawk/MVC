#[macro_use]
extern crate rocket;
mod db;
use db::{User, DB};
use rocket::{http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};
use user::user_provider::UserProvider;

mod user;

#[derive(Serialize)]
struct Error {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct CreateUser {
    name: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct UpdateUser {
    name: String,
    stars: i32,
}

#[get("/<name>")]
fn get_user(name: &str) -> Result<Json<User>, (Status, Json<Error>)> {
    let res = DB::new().unwrap().init().get_user(name);

    match res {
        Ok(us) => Ok(Json(us)),
        Err(_) => Err((
            Status::NotFound,
            Json(Error {
                message: "not found".to_string(),
            }),
        )),
    }
}

#[post("/", format = "application/json", data = "<user>")]
fn create_user(user: Json<CreateUser>) -> Result<Json<User>, (Status, Json<Error>)> {
    let conn = DB::new().unwrap();
    conn.create_user(&user.name);

    let res = conn.get_user(&user.name);

    match res {
        Ok(us) => Ok(Json(us)),
        Err(_) => Err((
            Status::NotFound,
            Json(Error {
                message: "not found".to_string(),
            }),
        )),
    }
}

#[delete("/<name>")]
fn delete_user(name: &str) -> Json<bool> {
    let conn = DB::new().unwrap();
    conn.delete_user(name);

    Json(true)
}

#[put("/<name>", format = "application/json", data = "<user>")]
fn update_user(name: &str, user: Json<UpdateUser>) -> Result<Json<User>, (Status, Json<Error>)> {
    let res = DB::new()
        .unwrap()
        .init()
        .update_user(name, &user.name, user.stars);

    match res {
        Ok(us) => Ok(Json(us)),
        Err(_) => Err((
            Status::NotFound,
            Json(Error {
                message: "not found".to_string(),
            }),
        )),
    }
}

#[launch]
fn rocket() -> _ {
    DB::new().unwrap().init();

    let usrProv = UserProvider::new();

    rocket::build().mount(
        "/user",
        routes![get_user, create_user, delete_user, update_user],
    )
}
