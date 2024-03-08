extern crate rocket;

use rocket::{ 
  form::Form,
  http::Status,
  response::{ status, Redirect },
  serde::{ json, Deserialize, Serialize },
  tokio::task::spawn_blocking,
  fs::TempFile,
};

#[derive(FromForm)]
struct FormData<'a>{
  file :Tempfile<'a>,
  secret:&'a str
}
