use actix_web::{web, App, HttpServer, HttpResponse};

use std::sync::atomic::{AtomicU32, Ordering};

struct AppStateWithCounter {
    counter: AtomicU32, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> actix_web::Result<HttpResponse>  {
    let current = data.counter.fetch_add(1, Ordering::Relaxed);

    Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(format!("Täiesti suvaline!\nKülastaja number: {}", current + 1)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: AtomicU32::new(0),
    });

    println!("Server started!");
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .workers(4) // <- Explicit worker count
    .run()
    .await
}
