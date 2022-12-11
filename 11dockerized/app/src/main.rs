extern crate core;

// rocket v0.5.0-rc.1
use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};

mod cors_config;
mod image_utils;

use cors_config::{CORS};
use image_utils::image_to_base64;


#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    age: u8,
}

#[derive(Deserialize, Serialize)]
struct ImageData {
    data: String,
}


#[rocket::post("/post", format = "json", data = "<user>")]
fn post_data(user: Json<User>) -> Json<User> { 
    let name: String = user.name.clone();
    let age: u8 = user.age.clone();

    Json(User { name, age })
}


#[rocket::get("/get", format = "json")]
fn get_data() -> Json<User> {
    let name: String = "My name".to_owned();
    let age: u8 = 8;

    return Json(User { name, age });
}


#[rocket::get("/get-image", format = "json")]
fn get_image() -> Json<ImageData> {
    let image: String = image_to_base64("assets/jellyfish.png".to_owned());
    // let img: String = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVQYV2NgYAAAAAMAAWgmWQ0AAAAASUVORK5CYII=".to_string();
    return Json(ImageData { data: image });
}


#[rocket::main]
async fn main() {
    if let Err(err) = rocket::build()
        .attach(CORS)
        .mount("/", rocket::routes![post_data, get_data, get_image])
        .launch()
        .await
    {
        println!("Rocket Rust couldn't take off successfully!");
        drop(err); // Drop initiates Rocket-formatted panic
    }
}