#[macro_use]
extern crate rocket;
extern crate clap;

use clap::{App, Arg};
use std::path::Path;
use std::process::Command;

struct Docx(Vec<u8>);

struct Config {
    base_url: String,
    base_file: Box<Path>,
    morpher_path: Box<Path>,
}

//TODO: make async
#[get("/<_..>/<cid>/<uid>")]
fn hello(cid: &str, uid: &str, config: &rocket::State<Config>) -> Docx {
    let morpher: &Path = &config.morpher_path;
    let dot_morpher = &Path::new("./").join(morpher);
    let morpher = match !(morpher.is_absolute() || morpher.starts_with("./")) {
        true => dot_morpher,
        _ => morpher,
    };
    // TODO: check return value
    Docx(
        Command::new(morpher)
            .arg(config.base_file.to_str().unwrap())
            .arg(format!("{}/{}/{}", &config.base_url, cid, uid))
            .output()
            .expect("Failed to execute command")
            .stdout,
    )
}

impl<'r> rocket::response::Responder<'r, 'static> for Docx {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            //TODO: make name dynamic
            //TODO: add content-type
            .raw_header("Content-Disposition", r#"attachment; filename="test.docm""#)
            .sized_body(self.0.len(), std::io::Cursor::new(self.0))
            .ok()
    }
}

fn validate_file_exists(s: String) -> Result<(), String> {
    match Path::new(&s).is_file() {
        true => Ok(()),
        _ => Err(format!("Cannot find file '{}'", s)),
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
        .arg(
            Arg::with_name("base_file")
                .long("base-file")
                .takes_value(true)
                .required(true)
                .validator(validate_file_exists),
        )
        .arg(
            Arg::with_name("file_morpher")
                .long("morpher")
                .takes_value(true)
                .required(true)
                .validator(validate_file_exists),
        )
        .get_matches();

    let config = Config {
        base_url: args.value_of("base_url").unwrap().to_string(),
        base_file: Box::from(Path::new(args.value_of("base_file").unwrap())),
        morpher_path: Box::from(Path::new(args.value_of("file_morpher").unwrap())),
    };

    rocket::build().manage(config).mount("/", routes![hello])
}
