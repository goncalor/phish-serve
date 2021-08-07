#[macro_use]
extern crate rocket;
use std::process::Command;

struct Docx(Vec<u8>);

//TODO: make async
#[get("/<_..>/<cid>/<uid>")]
fn hello(cid: &str, uid: &str) -> Docx {
    Docx(
        Command::new("./docm-morph.py")
            .arg("doc-samples/Anexo.docm")
            .arg("http://example.com")
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
    rocket::build().mount("/", routes![hello])
}
