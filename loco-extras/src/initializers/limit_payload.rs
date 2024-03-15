use axum::extract::DefaultBodyLimit;
use axum::Router as AxumRouter;
use loco_rs::app::AppContext;
use loco_rs::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct LimitPayloadInitializer;

#[async_trait]
impl Initializer for LimitPayloadInitializer {
    fn name(&self) -> String {
        "limit-payload".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        let router = ctx
            .config
            .server
            .middlewares
            .limit_payload
            .as_ref()
            .filter(|limit_payload| limit_payload.enable)
            .map(|limit_payload| -> Result<DefaultBodyLimit> {
                let middleware = DefaultBodyLimit::max(
                    byte_unit::Byte::from_str(&limit_payload.body_limit)
                        .map_err(Box::from)?
                        .get_bytes() as usize,
                );
                tracing::info!(
                    data = &limit_payload.body_limit,
                    "[Middleware] Adding {} layer",
                    self.name()
                );

                Ok(middleware)
            })
            .into_iter()
            .try_fold(router, |router, limit_payload| -> Result<AxumRouter> {
                Ok(router.layer(limit_payload?))
            })?;

        Ok(router)
    }
}
