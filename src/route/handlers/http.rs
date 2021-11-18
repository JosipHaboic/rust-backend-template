use actix_web::{HttpRequest, HttpResponse, Result, Error};
use actix_web::web;
use tera::Context;

use crate::template;
use crate::state;



pub async fn index(_req: HttpRequest, data: web::Data<state::Application>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("title", &data.title);
    context.insert("version", &data.version);

    Ok(HttpResponse::Ok().body(template::TEMPLATES.render("index.html", &context).unwrap()))
}

pub async fn about(_req: HttpRequest,  data: web::Data<state::Application>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("title", &data.title);
    context.insert("version", &data.version);

    Ok(HttpResponse::Ok().body(template::TEMPLATES.render("about.html", &context).unwrap()))
}