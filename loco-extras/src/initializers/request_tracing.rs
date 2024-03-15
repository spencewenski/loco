use async_trait::async_trait;
use axum::middleware::AddExtension;
use axum::{http, Router as AxumRouter};
use loco_rs::environment::Environment;
use loco_rs::prelude::*;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::classify::MakeClassifier;
use tower_http::trace::TraceLayer;

#[allow(clippy::module_name_repetitions)]
pub struct RequestTracingInitializer;

#[async_trait]
impl Initializer for RequestTracingInitializer {
    fn name(&self) -> String {
        "request-tracing".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .middlewares
            .logger
            .as_ref()
            .filter(|logger| logger.enable)
            .map(|_| {
                TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                    let request_id = uuid::Uuid::new_v4();
                    let user_agent = request
                        .headers()
                        .get(axum::http::header::USER_AGENT)
                        .map_or("", |h| h.to_str().unwrap_or(""));

                    let env: String = request
                        .extensions()
                        .get::<Environment>()
                        .map(std::string::ToString::to_string)
                        .unwrap_or_default();

                    tracing::error_span!(
                        "http-request",
                        "http.method" = %request.method(),
                        "http.uri" = %request.uri(),
                        "http.version" = ?request.version(),
                        "http.user_agent" = %user_agent,
                        "environment" = %env,
                        request_id = %request_id,
                    )
                })
            })
            .into_iter()
            .fold(router, |router, trace_layer| {
                tracing::info!("[Middleware] Adding {} layer", self.name());
                router
                    .layer(trace_layer)
                    .layer(AddExtensionLayer::new(ctx.environment.clone()))
            });

        Ok(router)
    }
}
