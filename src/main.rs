#[macro_use]
extern crate rocket;
extern crate clap;

use clap::{App, Arg};
use rocket::http::Status;
use std::path::Path;
use std::process::Command;

struct Docx {
    name: String,
    content: Vec<u8>,
}

struct Config {
    base_url: String,
    base_file: Box<Path>,
    morpher_path: Box<Path>,
}

// This is needed because serve_ignored() requires an ignored segment
#[get("/<cid>/<uid>", rank = 0)]
fn serve(cid: &str, uid: &str, config: &rocket::State<Config>) -> Result<Docx, Status> {
    serve_ignored(cid, uid, config)
}

#[get("/<_..>/<cid>/<uid>", rank = 1)]
fn serve_ignored<'a>(cid: &str, uid: &str, config: &rocket::State<Config>) -> Result<Docx, Status> {
    let morpher: &Path = &config.morpher_path;
    let dot_morpher = &Path::new("./").join(morpher);
    let morpher = match !(morpher.is_absolute() || morpher.starts_with("./")) {
        true => dot_morpher,
        _ => morpher,
    };

    let output = Command::new(morpher)
        .arg(config.base_file.to_str().unwrap())
        .arg(format!("{}/{}/{}", &config.base_url, cid, uid))
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        return Err(Status::InternalServerError);
    }

    Ok(Docx {
        // same name as base file name
        name: config
            .base_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        content: output.stdout,
    })
}

impl<'r> rocket::response::Responder<'r, 'static> for Docx {
    fn respond_to(self, _: &rocket::Request<'_>) -> rocket::response::Result<'static> {
        rocket::Response::build()
            //TODO: add content-type
            .raw_header(
                "Content-Disposition",
                format!(r#"attachment; filename="{}""#, self.name),
            )
            .sized_body(self.content.len(), std::io::Cursor::new(self.content))
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

    rocket::build()
        .manage(config)
        .mount("/", routes![serve_ignored, serve])
}
