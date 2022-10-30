use rocket::{get, launch, routes, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Greeting {
    name: String,
    greeting: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/greet/<name>")]
fn greet(name: &str) -> Json<Greeting> {
    Json(Greeting {
        name: name.to_string(),
        greeting: format!("Hello, {}!", name),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, greet])
}
