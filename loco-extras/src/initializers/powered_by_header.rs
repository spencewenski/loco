use async_trait::async_trait;
use axum::{http, Router as AxumRouter};
use loco_rs::prelude::*;
use tower_http::set_header::SetResponseHeaderLayer;

#[allow(clippy::module_name_repetitions)]
pub struct PoweredByHeaderInitializer;

#[async_trait]
impl Initializer for PoweredByHeaderInitializer {
    fn name(&self) -> String {
        "powered-by-header".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .ident
            .as_ref()
            .and_then(|ident| {
                if ident.is_empty() {
                    None
                } else {
                    Some("loco.rs")
                }
            })
            .map(http::header::HeaderValue::from_str)
            .transpose()?
            .map(|header_value| {
                SetResponseHeaderLayer::overriding(
                    http::header::HeaderName::from_static("x-powered-by"),
                    header_value,
                )
            })
            .into_iter()
            .fold(router, |router, powered_by| {
                tracing::info!("[Middleware] Adding {} layer", self.name());
                router.layer(powered_by)
            });

        Ok(router)
    }
}
