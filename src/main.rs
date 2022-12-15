use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use std::fs::File;
use serde_json::Value;

struct AppState {
    app_name: Value,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!, Usage: {URL}/code/postal_code ")
}

#[get("/code/{postal_code}")]
async fn postal_code(req: HttpRequest, data: web::Data<AppState>) -> Result<String> {
    let userid: String = req.match_info().query("postal_code").parse().unwrap();
    let info = &data.app_name.get(userid)
    .expect("The postal_code is not in the database");
    Ok(format!("{}",info))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {


    HttpServer::new(|| {

        let file:File = File::open("codigos_postales_reduced.json")
             .expect("file should open read only");
        let json: serde_json::Value = serde_json::from_reader(file)
            .expect("file should be proper JSON");
        let postal_code_db:&Value = json.get("codigos_postales")
            .expect("file should have FirstName key");

        App::new()
            .app_data( web::Data::new(AppState {
                app_name: postal_code_db.clone()}  ))
            .service(hello)
            .service(postal_code)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}