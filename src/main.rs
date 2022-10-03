use actix_web::{get, web, web::Data, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use tera::{Context, Tera};

#[get("/hello/{name}")]
async fn render_tmpl(name: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("name", &name.clone());
    ctx.insert("title", &name.into_inner());
    ctx.insert("stylesheet", "/static/main.css");
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[derive(Clone)]
struct AppData {
    tmpl: Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera_templates_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*");

    let data = Data::new(AppData {tmpl: Tera::new(tera_templates_dir).unwrap()});

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(render_tmpl)
            .service(fs::Files::new("/static", "public"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
