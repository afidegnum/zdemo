use moon::{
    actix_web::{
        get,
        http::StatusCode,
        middleware::{Compat, Condition, ErrorHandlers, Logger},
        web::ServiceConfig,
        App, Responder,
    },
    config::CONFIG,
    *,
};

pub mod error_handler;

async fn frontend() -> Frontend {
    Frontend::new()
        .title("Counter example")
        .default_styles(false)
        .body_content("")
        .append_to_head(r#"<link href="/_api/public/css/styles.css" rel="stylesheet"/>"#)
        .append_to_head(r#"<link href="/_api/public/css/bootstrap/dist/css/bootstrap.min.3.4.0.css" rel="stylesheet"/>"#)
}

async fn up_msg_handler(_: UpMsgRequest<()>) {}

#[get("hello")]
async fn hello() -> impl Responder {
    "Hello!"
}

#[get("hi")]
async fn hi() -> impl Responder {
    "Hello and Hi"
}

#[moon::main]
async fn main() -> std::io::Result<()> {
    let app = || {
        let redirect = Redirect::new()
            .http_to_https(CONFIG.https)
            .port(CONFIG.redirect.port, CONFIG.port);

        App::new()
            .wrap(Condition::new(
                CONFIG.redirect.enabled,
                Compat::new(redirect),
            ))
            .wrap(Logger::new("%r %s %D ms %a"))
            .wrap(
                ErrorHandlers::new()
                    .handler(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        error_handler::internal_server_error,
                    )
                    .handler(StatusCode::NOT_FOUND, error_handler::not_found),
            )
    };

    let service_config = |service_config: &mut ServiceConfig| {
        service_config.service(hello);
        service_config.service(hi);
    };

    start_with_app(frontend, up_msg_handler, app, service_config).await
}
