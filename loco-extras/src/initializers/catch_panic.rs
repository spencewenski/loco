use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::controller::middleware::catch_panic::custom_catch_panic_layer;
use loco_rs::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct CatchPanicInitializer;

#[async_trait]
impl Initializer for CatchPanicInitializer {
    fn name(&self) -> String {
        "catch-panic".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .middlewares
            .catch_panic
            .as_ref()
            .filter(|catch_panic| catch_panic.enable)
            .map(|_| custom_catch_panic_layer())
            .into_iter()
            .fold(router, |router, catch_panic_layer| {
                tracing::info!("[Middleware] Adding {} layer", self.name());
                router.layer(catch_panic_layer)
            });

        Ok(router)
    }
}
