use crate::authentication::reject_anonymous_users;
use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
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
use actix_web::middleware::from_fn;
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
        let email_client = configuration.email_client.client();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
            email_client,
            configuration.application.base_url,
            configuration.application.hmac_secret,
            configuration.redis_uri,
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
        .append_header(("Location", "/swagger-ui/"))
        .finish()
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
    hmac_secret: SecretString,
    redis_uri: SecretString,
) -> Result<Server, anyhow::Error> {
    #[derive(OpenApi)]
    #[openapi(
        nest((path = "/api/v1", api = crate::routes::api::v1::Apiv1)),
        tags((name = "webserver api", description = "core webserver api for sharing and evaluating code")),
        )]
    struct ApiDoc;

    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
    let redis_store = RedisSessionStore::new(redis_uri.expose_secret()).await?;

    let openapi = ApiDoc::openapi();
    let server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(TracingLogger::default())
            .route("/", web::get().to(home))
            .service(
                web::scope("/api/v1")
                    .service(get_share_code)
                    .service(post_share_code)
                    .service(
                        web::scope("/evaluate")
                            .service(post_with_code_and_platform)
                            .service(post_with_code_and_clex)
                            .service(post_with_code_and_constraint)
                            .service(post_with_platform)
                            .service(post_with_shared_id),
                    ),
            )
            .service(web::scope("/admin").wrap(from_fn(reject_anonymous_users)));
        if std::env::var("APP_ENVIRONMENT").unwrap_or_default() != "production" {
            app = app
                .service(Redoc::with_url("/redoc", openapi.clone()))
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", openapi.clone()),
                )
                .route("/swagger-ui", web::get().to(redirect_swagger_ui))
                // There is no need to create RapiDoc::with_openapi because the OpenApi is served
                // via SwaggerUi. Instead we only make rapidoc to point to the existing doc.
                //
                // If we wanted to serve the schema, the following would work:
                // .service(RapiDoc::with_openapi("/api-docs/openapi2.json", openapi.clone()).path("/rapidoc"))
                .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
                .service(Scalar::with_url("/scalar", openapi.clone()))
        }
        app.route("/health_check", web::get().to(health_check))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

#[derive(Clone)]
pub struct HmacSecret(pub SecretString);
