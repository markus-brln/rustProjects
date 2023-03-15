extern crate core;

// rocket v0.5.0-rc.1
use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};

#[path = "./api/user.rs"]
mod user;
use user::{
    post_json_data,
    get_json_data
};

mod cors_config;
use cors_config::{CORS};

mod image_utils;
use image_utils::open_image_to_base64;


#[derive(Deserialize, Serialize)]
struct ImageData {
    data: String,
}


#[rocket::get("/get-image", format = "json")]
fn get_image() -> Json<ImageData> {
    let image: String = open_image_to_base64("assets/jellyfish.png".to_owned());
    // let img: String = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVQYV2NgYAAAAAMAAWgmWQ0AAAAASUVORK5CYII=".to_string();
    return Json(ImageData { data: image });
}


#[rocket::main]
async fn main() {
    if let Err(err) = rocket::build()
        .attach(CORS)
        .mount("/", rocket::routes![post_json_data, get_json_data, get_image])
        .launch()
        .await
    {
        println!("Rocket Rust couldn't take off successfully!");
        drop(err); // Drop initiates Rocket-formatted panic
    }
}