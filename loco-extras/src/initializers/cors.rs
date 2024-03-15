use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;
use std::time::Duration;
use tower_http::cors::CorsLayer;

#[allow(clippy::module_name_repetitions)]
pub struct CorsInitializer;

#[async_trait]
impl Initializer for CorsInitializer {
    fn name(&self) -> String {
        "cors".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .middlewares
            .cors
            .as_ref()
            .filter(|cors_middleware| cors_middleware.enable)
            .map(|cors_middleware| -> Result<CorsLayer> {
                // Todo: is CorsLayer::permissive() the default if cors is not enabled?
                let mut cors: CorsLayer = CorsLayer::permissive();

                if let Some(allow_origins) = &cors_middleware.allow_origins {
                    // testing CORS, assuming https://example.com in the allow list:
                    // $ curl -v --request OPTIONS 'localhost:3000/api/_ping' -H 'Origin: https://example.com' -H 'Access-Control-Request-Method: GET'
                    // look for '< access-control-allow-origin: https://example.com' in response.
                    // if it doesn't appear (test with a bogus domain), it is not allowed.
                    let mut list = vec![];
                    for origins in allow_origins {
                        list.push(origins.parse()?);
                    }
                    cors = cors.allow_origin(list);
                }

                if let Some(allow_headers) = &cors_middleware.allow_headers {
                    let mut headers = vec![];
                    for header in allow_headers {
                        headers.push(header.parse()?);
                    }
                    cors = cors.allow_headers(headers);
                }

                if let Some(allow_methods) = &cors_middleware.allow_methods {
                    let mut methods = vec![];
                    for method in allow_methods {
                        methods.push(method.parse()?);
                    }
                    cors = cors.allow_methods(methods);
                }

                if let Some(max_age) = cors_middleware.max_age {
                    cors = cors.max_age(Duration::from_secs(max_age));
                }

                Ok(cors)
            })
            .into_iter()
            .try_fold(router, |router, cors| -> Result<AxumRouter> {
                tracing::info!("[Middleware] Adding {} layer", self.name());
                Ok(router.layer(cors?))
            })?;
        Ok(router)
    }
}
