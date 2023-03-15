// rocket v0.5.0-rc.1
use rocket::{
    self,
    serde::{json::Json, Deserialize, Serialize},
};


#[derive(Deserialize, Serialize)]
pub struct User {
    name: String,
    age: u8,
}

#[rocket::post("/post-json", format = "json", data = "<user>")]
pub fn post_json_data(user: Json<User>) -> Json<User> {
    let name: String = user.name.clone();
    let age: u8 = user.age.clone();

    return Json(User { name, age })
}


#[rocket::get("/get-json", format = "json")]
pub fn get_json_data() -> Json<User> {
    let name: String = "My name".to_owned();
    let age: u8 = 8;

    return Json(User { name, age });
}

