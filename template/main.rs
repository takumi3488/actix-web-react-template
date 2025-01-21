use actix_files as fs;
use actix_web::{get, App, HttpServer};
use anyhow::Result;
use std::path::Path;
use tracing::debug;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    HttpServer::new(|| {
        let mut app = App::new().wrap(TracingLogger::default()).service(health);
        if Path::new("./dist").is_dir() {
            app = app.service(fs::Files::new("/", "./dist").index_file("index.html"));
        }
        app
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}

#[get("/api/health")]
async fn health() -> String {
    debug!("health check");
    "OK".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health() {
        let app = test::init_service(App::new().service(health)).await;
        let req = test::TestRequest::get().uri("/api/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
