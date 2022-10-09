use actix_cors::Cors;
use actix_web::{
    App,HttpServer,Responder,HttpResponse,get,
    middleware::{
        Compress,
        Logger,
    },
    web::{
        self,
        Data,
    },
};
use anyhow::Result;
use dotenv::dotenv;
use graphql::{
    graphiql,
    graphql,
    playground,
    schemas::create_schema,
};
use std::{
    env,
    sync::Arc,
};

#[get("/")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    // .envに記述された環境変数の読み込み.
    dotenv().ok();

    // debugと同等以上の重要度を持つログを表示するように設定し、ログを開始する.
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Schemaオブジェクトをスレッドセーフな型でホランラップする.
    let schema = Arc::new(create_schema());

    // サーバーの色んな設定.
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .supports_credentials()
            )
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(health)
            .service(
                web::resource("/graphql")
                    .route(web::get().to(graphql))
                    .route(web::post().to(graphql))
            )
            // /graphiqlエンドポイントにgraphiql()をセットする.
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
            // /playgroundエンドポイントにplayground()をセットする.
            .service(web::resource("/playground").route(web::get().to(playground)))
    });
    server = server.bind(("0.0.0.0", 8000))?;
    server.run().await?;

    Ok(())
}