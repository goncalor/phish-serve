#[macro_use]
extern crate rocket;

#[get("/<_..>/<cid>/<uid>")]
fn hello(cid: &str, uid: &str) -> String {
    format!("{} {}", cid, uid)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
