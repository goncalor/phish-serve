#[macro_use]
extern crate rocket;
use std::process::Command;

#[get("/<_..>/<cid>/<uid>")]
fn hello(cid: &str, uid: &str) -> Vec<u8> {
    Command::new("./docm-morph.py")
        .arg("doc-samples/Anexo.docm")
        .arg("http://example.com")
        .output()
        .expect("Failed to execute command").stdout
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
