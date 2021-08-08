#[macro_use]
extern crate rocket;
extern crate clap;

use clap::{App, Arg};
use std::process::Command;

struct Docx(Vec<u8>);

struct Config {
    base_url: String,
    base_file: String,
    morpher_path: String,
}

//TODO: make async
#[get("/<_..>/<cid>/<uid>")]
fn hello(cid: &str, uid: &str, config: &rocket::State<Config>) -> Docx {
    Docx(
        Command::new("./docm-morph.py")
            .arg("doc-samples/Anexo.docm")
            .arg(&config.base_url)
            .output()
            .expect("Failed to execute command")
            .stdout,
    )
}

impl<'r> rocket::response::Responder<'r, 'static> for Docx {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            //TODO: make name dynamic
            .raw_header("Content-Disposition", r#"attachment; filename="test.docm""#)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

#[launch]
fn rocket() -> _ {
    let args = App::new("phish-serve")
        .arg(
            Arg::with_name("base_url")
                .long("base-url")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let config = Config {
        base_url: args.value_of("base_url").unwrap().to_string(),
        base_file: String::from(""),
        morpher_path: String::from(""),
    };

    rocket::build().manage(config).mount("/", routes![hello])
}
