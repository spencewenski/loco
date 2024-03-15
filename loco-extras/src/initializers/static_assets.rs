use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;
use std::path::PathBuf;
use tower_http::services::{ServeDir, ServeFile};

#[allow(clippy::module_name_repetitions)]
pub struct StaticAssetsInitializer;

#[async_trait]
impl Initializer for StaticAssetsInitializer {
    fn name(&self) -> String {
        "static-assets".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .middlewares
            .static_assets
            .as_ref()
            .filter(|config| config.enable)
            .map(|config| {
                tracing::info!("[Middleware] Adding {} layer", self.name());
                if config.must_exist
                    && (!PathBuf::from(&config.folder.path).exists()
                        || !PathBuf::from(&config.fallback).exists())
                {
                    return Err(Error::Message(format!(
                        "one of the static path are not found, Folder `{}` fallback: `{}`",
                        config.folder.path, config.fallback,
                    )));
                }

                let serve_dir = ServeDir::new(&config.folder.path)
                    .not_found_service(ServeFile::new(&config.fallback));

                let service = if config.precompressed {
                    tracing::info!("[Middleware] Enable precompressed static assets");
                    serve_dir.precompressed_gzip()
                } else {
                    serve_dir
                };

                Ok((config, service))
            })
            .into_iter()
            .try_fold(router, |router, static_assets| -> Result<AxumRouter> {
                let (config, service) = static_assets?;
                Ok(router.nest_service(&config.folder.uri, service))
            })?;

        Ok(router)
    }
}
