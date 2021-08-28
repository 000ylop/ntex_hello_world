use ntex::web::{self, middleware, App};

#[ntex::main]
async fn main() -> std::io::Result<()> {

    web::server(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service((
                web::resource("/").to(|| async { "Hello World!\n" }),
            ))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// code from https://zhuanlan.zhihu.com/p/367098988