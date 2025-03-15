use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::api::v1::evaluate::with_code_and_clex::post_with_code_and_clex;
use crate::routes::api::v1::evaluate::with_code_and_constraint::post_with_code_and_constraint;
use crate::routes::api::v1::evaluate::with_code_and_platform::post_with_code_and_platform;
use crate::routes::api::v1::evaluate::with_platform::post_with_platform;
use crate::routes::api::v1::evaluate::with_shared_id::post_with_shared_id;
use crate::routes::api::v1::share::get::get_share_code;
use crate::routes::api::v1::share::post::post_share_code;
use crate::routes::{health_check, home};
use actix_session::SessionMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_flash_messages::storage::CookieMessageStore;
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
            configuration.llm.api_key,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

pub struct ApplicationBaseUrl(pub String);

async fn redirect_swagger_ui() -> impl Responder {
    HttpResponse::Found()
        .append_header(("Location", "/api/v1/docs/swagger-ui/"))
        .finish()
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    base_url: String,
    hmac_secret: SecretString,
    redis_uri: SecretString,
    gemini_api_key: SecretString,
) -> Result<Server, anyhow::Error> {
    #[derive(OpenApi)]
    #[openapi(
        nest((path = "/api/v1", api = crate::routes::api::v1::Apiv1)),
        tags((name = "webserver api", description = "core webserver api for sharing and evaluating code")),
        )]
    struct ApiDoc;

    let db_pool = Data::new(db_pool);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let gemini_api_key = Data::new(gemini_api_key);
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let openapi = ApiDoc::openapi();

    let server = HttpServer::new(move || {
        let mut api_v1 = web::scope("/api/v1")
            .service(get_share_code)
            .service(post_share_code)
            .service(
                web::scope("/evaluate")
                    .service(post_with_shared_id)
                    .service(post_with_code_and_clex)
                    .app_data(gemini_api_key.clone())
                    .service(post_with_code_and_platform)
                    .service(post_with_code_and_constraint)
                    .service(post_with_platform),
            );

        if std::env::var("APP_ENVIRONMENT").unwrap_or_default() != "production" {
            api_v1 = api_v1.service(
                web::scope("/docs")
                    .service(Redoc::with_url("/redoc", openapi.clone()))
                    .service(RapiDoc::with_openapi("/openapi.json", openapi.clone()))
                    .service(
                        SwaggerUi::new("/swagger-ui/{_:.*}")
                            .url("/api/v1/docs/openapi.json", openapi.clone()),
                    )
                    .route("/swagger-ui", web::get().to(redirect_swagger_ui))
                    .service(RapiDoc::new("/api/v1/docs/openapi.json").path("/rapidoc"))
                    .service(Scalar::with_url("/scalar", openapi.clone())),
            );
        }

        let app = App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .service(api_v1);
        app.route("/health_check", web::get().to(health_check))
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub SecretString);
