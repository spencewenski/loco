use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;
use tower_http::compression::CompressionLayer;

#[allow(clippy::module_name_repetitions)]
pub struct CompressionInitializer;

#[async_trait]
impl Initializer for CompressionInitializer {
    fn name(&self) -> String {
        "compression".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .middlewares
            .compression
            .as_ref()
            .filter(|compression| compression.enable)
            .map(|_| CompressionLayer::new())
            .into_iter()
            .fold(router, |router, compression| {
                tracing::info!("[Middleware] Adding {} layer", self.name());
                router.layer(compression)
            });

        Ok(router)
    }
}
