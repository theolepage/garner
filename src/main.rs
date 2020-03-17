#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate nanoid;
use std::io;
use std::path::Path;
use rocket::Data;
use rocket::response::NamedFile;
use nanoid::nanoid;

#[get("/")]
fn usage() -> String {
    format!("Usage: ")
}

#[post("/", data = "<data>")]
fn post_file(data: Data) -> io::Result<String> {
    let id = nanoid!(4);
    let filename = format!("./uploads/{}", id);

    data.stream_to_file(Path::new(&filename))?;

    Ok(format!("Pushed file {}", filename))
}

#[get("/<id>")]
fn get_file(id: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("./uploads/").join(id)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![usage, get_file, post_file])
        .launch();
}