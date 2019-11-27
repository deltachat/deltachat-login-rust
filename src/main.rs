extern crate handlebars;
#[macro_use]
extern crate serde_json;

use handlebars::Handlebars;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use std::io;

fn main() -> io::Result<()> {
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .register_data(handlebars_ref.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
}

fn index(hb: web::Data<Handlebars>) -> impl Responder {

    let data = json!({
        "qr_code": "[QR]",
        "qr_code_url": "OPENPGP4FPR:..."
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

//#[get("/{user}/{data}")]
//fn user(hb: web::Data<Handlebars>, info: web::Path<(String, String)>) -> HttpResponse {
//    let data = json!({
//        "user": info.0,
//        "data": info.1
//    });
//    let body = hb.render("user", &data).unwrap();
//
//    HttpResponse::Ok().body(body)
//}

