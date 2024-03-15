//! [Initializer] to add the [EtagLayer] middleware to the [AxumRouter].

use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::controller::middleware::etag::EtagLayer;
use loco_rs::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct EtagInitializer;

#[async_trait]
impl Initializer for EtagInitializer {
    fn name(&self) -> String {
        "etag".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .middlewares
            .etag
            .as_ref()
            .filter(|enable_middleware| enable_middleware.enable)
            .map(|_| EtagLayer::new())
            .into_iter()
            .fold(router, |router, middleware| {
                tracing::info!("[Middleware] Adding {} layer", self.name());
                router.layer(middleware)
            });

        Ok(router)
    }
}
