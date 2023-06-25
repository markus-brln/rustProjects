extern crate core;

// rocket v0.5.0-rc.1
use rocket::{
    self,
    // serde::{json::Json, Deserialize, Serialize},
};

mod cors_config;

use cors_config::{CORS};


// #[derive(Deserialize, Serialize)]
// struct User {
//     name: String,
//     age: u8,
// }


// #[rocket::get("/get", format = "json")]
// fn get_data() -> Json<User> {
//     let name: String = "My name".to_owned();
//     let age: u8 = 8;
//
//     return Json(User { name, age });
// }

#[rocket::get("/hello")]
fn hello() -> String {
    format!("Hello!")
}


#[rocket::main]
async fn main() {
    if let Err(err) = rocket::build()
        .attach(CORS)
        .mount("/", rocket::routes![hello]) // get_data])
        .launch()
        .await
    {
        println!("Rocket Rust couldn't take off successfully!");
        drop(err); // Drop initiates Rocket-formatted panic
    }
}