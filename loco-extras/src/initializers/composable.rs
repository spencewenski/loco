use crate::initializers::catch_panic::CatchPanicInitializer;
use crate::initializers::compression::CompressionInitializer;
use crate::initializers::cors::CorsInitializer;
use crate::initializers::etag::EtagInitializer;
use crate::initializers::limit_payload::LimitPayloadInitializer;
use crate::initializers::powered_by_header::PoweredByHeaderInitializer;
use crate::initializers::request_tracing::RequestTracingInitializer;
use crate::initializers::static_assets::StaticAssetsInitializer;
use crate::initializers::timeout::TimeoutInitializer;
use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::prelude::*;

#[allow(clippy::module_name_repetitions)]
pub struct ComposableInitializer {
    initializers: Vec<Box<dyn Initializer>>,
}

impl ComposableInitializer {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            initializers: vec![],
        }
    }

    pub fn add_defaults(self) -> Result<Self> {
        // Todo: check the order
        let initializers: Vec<Box<dyn Initializer>> = vec![
            Box::new(RequestTracingInitializer),
            Box::new(LimitPayloadInitializer),
            Box::new(CorsInitializer),
            Box::new(CatchPanicInitializer),
            Box::new(CompressionInitializer),
            Box::new(StaticAssetsInitializer),
            Box::new(TimeoutInitializer),
            Box::new(EtagInitializer),
            Box::new(PoweredByHeaderInitializer),
        ];
        self.append_all(initializers)
    }

    pub fn prepend(mut self, initializer: Box<dyn Initializer>) -> Result<Self> {
        self.check_initializer_not_added(&*initializer)?;
        self.initializers.insert(0, initializer);
        Ok(self)
    }

    pub fn append(mut self, initializer: Box<dyn Initializer>) -> Result<Self> {
        self.check_initializer_not_added(&*initializer)?;
        self.initializers.push(initializer);
        Ok(self)
    }

    pub fn prepend_all(self, initializers: Vec<Box<dyn Initializer>>) -> Result<Self> {
        initializers
            .into_iter()
            .try_fold(self, |bulk, initializer| bulk.prepend(initializer))
    }

    pub fn append_all(self, initializers: Vec<Box<dyn Initializer>>) -> Result<Self> {
        initializers
            .into_iter()
            .try_fold(self, |bulk, initializer| bulk.append(initializer))
    }

    fn check_initializer_not_added(&self, initializer: &dyn Initializer) -> Result<()> {
        if self
            .initializers
            .iter()
            .any(|existing| existing.name() == initializer.name())
        {
            return Err(Error::Message(format!(
                "Initializer `{}` was already added",
                initializer.name(),
            )));
        }
        Ok(())
    }
}

#[async_trait]
impl Initializer for ComposableInitializer {
    fn name(&self) -> String {
        "bulk".to_string()
    }

    async fn before_run(&self, ctx: &AppContext) -> Result<()> {
        for initializer in self.initializers.iter() {
            initializer.before_run(ctx).await?
        }

        Ok(())
    }

    async fn after_routes(&self, mut router: AxumRouter, ctx: &AppContext) -> Result<AxumRouter> {
        // Reverse due to how adding middleware layers works in axum
        for initializer in self.initializers.iter().rev() {
            router = initializer.after_routes(router, ctx).await?
        }

        Ok(router)
    }
}
