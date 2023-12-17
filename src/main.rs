// This is a Rocket file.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, visitor!"
}
