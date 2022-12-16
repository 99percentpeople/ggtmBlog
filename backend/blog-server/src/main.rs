use std::{env, error::Error, path::Path, time::Duration};

use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_settings::{ApplySettings, BasicSettings, Mode};
use actix_web::{
    cookie,
    middleware::{Compress, Condition, Logger},
    web, App, HttpServer,
};
use actix_web_lab::web::spa;
use anyhow::Result;
use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;
pub mod dbaccess;
pub mod error;
pub mod handler;
pub mod model;
pub mod util;

#[macro_use]
extern crate lazy_static;
/// Application-specific settings
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct AppSettings {
    database_url: String,
    file_url: String,
}
/// Convenience type alias for [`BasicSettings`] with [`AppSettings`].
pub type Settings = BasicSettings<AppSettings>;

/// Initialize the logging infrastructure.
fn init_logger(settings: &Settings) {
    if !settings.actix.enable_log {
        return;
    }
    std::env::set_var(
        "RUST_LOG",
        match settings.actix.mode {
            Mode::Development => "debug",
            Mode::Production => "info",
        },
    );
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
}

fn build_indetity_service() -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(&[0; 32])
            .max_age(cookie::time::Duration::days(3))
            .login_deadline(cookie::time::Duration::days(3))
            .visit_deadline(cookie::time::Duration::days(1))
            .path("/")
            .secure(false)
            .http_only(true)
            .name("auth"),
    )
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_path =
        Path::new(&env::var("SERVER_CONFIG").unwrap_or("config.toml".to_owned())).to_owned();
    let mut settings = Settings::parse_toml(config_path.to_owned()).expect(&format!(
        "Failed to parse `Settings` from {}",
        config_path.to_str().unwrap_or_default()
    ));
    Settings::override_field_with_env_var(&mut settings.actix.mode, "MODE").ok();
    Settings::override_field_with_env_var(&mut settings.application.database_url, "DATABASE_URL")
        .ok();

    init_logger(&settings);

    let mut opt = ConnectOptions::new(settings.application.database_url.to_owned());
    opt.sqlx_logging(true) // Enable SQLx log
        .sqlx_logging_level(log::LevelFilter::Debug); // Setting SQLx log level
    let conn = Database::connect(opt).await?;

    let tls = settings.actix.tls.enabled;
    settings.actix.hosts.iter().for_each(move |v| {
        log::info!(
            "Starting server on {}",
            format!(
                "{}://{}:{}",
                if tls { "https" } else { "http" },
                v.host,
                v.port
            )
        )
    });

    HttpServer::new({
        let settings = settings.to_owned();
        move || {
            App::new()
                .app_data(web::Data::new(settings.to_owned()))
                .app_data(web::Data::new(conn.to_owned()))
                .wrap(Condition::new(
                    settings.actix.enable_compression,
                    Compress::default(),
                ))
                .wrap(Logger::default())
                .wrap(build_indetity_service())
                .service(
                    web::scope("/api")
                        .configure(handler::auth::auth_handler)
                        .service(web::scope("/sort").configure(handler::sort::sort_handler))
                        .service(web::scope("/tag").configure(handler::tag::tag_handler))
                        .service(web::scope("/blog").configure(handler::blog::blog_handler))
                        .service(
                            web::scope("/comment").configure(handler::comment::comment_handler),
                        )
                        .service(web::scope("/file").configure(handler::file::files_handler))
                        .wrap(
                            RateLimiter::builder(
                                InMemoryBackend::builder().build(),
                                SimpleInputFunctionBuilder::new(Duration::from_secs(60), 60)
                                    .real_ip_key()
                                    .build(),
                            )
                            .add_headers()
                            .build(),
                        )
                        .service(web::scope("/admin").service(
                            web::scope("/blog").configure(handler::admin_blog::admin_blog_handler),
                        )),
                )
                .configure(|cfg| match settings.actix.mode {
                    Mode::Development => (),
                    Mode::Production => {
                        cfg.service(
                            web::scope("/admin").service(
                                spa()
                                    .index_file("./www/blog-admin/index.html")
                                    .static_resources_mount("")
                                    .static_resources_location("./www/blog-admin")
                                    .finish(),
                            ),
                        )
                        .service(
                            spa()
                                .index_file("./www/blog/index.html")
                                .static_resources_mount("/")
                                .static_resources_location("./www/blog")
                                .finish(),
                        );
                    }
                })
        }
    })
    .apply_settings(&settings)
    .run()
    .await?;
    Ok(())
}
