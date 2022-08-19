#[macro_use]
extern crate rocket;

use pomsky_macro::pomsky;

#[get("/")]
fn index() -> &'static str {
    pomsky!(["great!"] | "great!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
