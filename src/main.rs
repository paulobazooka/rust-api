use actix_web::{App, HttpServer};

mod routes;
use routes::index::index::index;

const ADDRESS: &'static str = "0.0.0.0";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let api = HttpServer::new(|| {
        App::new()
            .service(index)
    });

    let api = api.bind((ADDRESS, PORT))
        .expect("Erro ao iniciar o servidor!");

    println!("-- server run {}:{} --", ADDRESS, PORT);

    api.run().await

}
