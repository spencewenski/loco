use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

#[allow(clippy::module_name_repetitions)]
pub struct TimeoutInitializer;

#[async_trait]
impl Initializer for TimeoutInitializer {
    fn name(&self) -> String {
        "timeout".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .middlewares
            .timeout_request
            .as_ref()
            .filter(|timeout_request| timeout_request.enable)
            .map(|timeout_request| {
                TimeoutLayer::new(Duration::from_millis(timeout_request.timeout))
            })
            .into_iter()
            .fold(router, |router, timeout_request| {
                tracing::info!("[Middleware] Adding {} layer", self.name());

                router.layer(timeout_request)
            });

        Ok(router)
    }
}
