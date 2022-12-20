use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use lazy_static::lazy_static;
use std::fs;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!, Usage: {URL}/code/postal_code ")
}

#[get("/show_path/{path}")]
async fn show_path(req: HttpRequest) -> Result<String> {
    let mut s:String = "".to_owned();
    let mut path: String = req.match_info().query("path").parse().unwrap();
    if path == "1" {
        path = "./".to_owned();
    }else if path == "2"{
        path = "../".to_owned();
    }else if path.contains("11") {
        path = path.replace("11", "/");
    }
    println!("{}",path);

    for file in fs::read_dir(path).unwrap() {
         //println!("{}", file.unwrap().path().display());
         s.push_str(file.unwrap().path().to_str().unwrap());
         s.push_str("\n ");

    }
    //println!("{}",s);
    Ok(format!("{}",s))
}



#[get("/code/{postal_code}")]
async fn postal_code(req: HttpRequest) -> Result<String> {
    let postal_code: String = req.match_info().query("postal_code").parse().unwrap();
    Ok(format!("{}",JSON["codigos_postales"][postal_code.clone()]))
}

lazy_static! {
    static ref JSON: serde_json::Value = {
        lazy_static! {
            static ref WJ: String = {
                            std::fs::read_to_string("codigos_postales_reduced.json").unwrap()
            };
        }
        serde_json::from_str(&WJ).unwrap()
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(postal_code)
            .service(show_path)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}