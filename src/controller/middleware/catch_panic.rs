use crate::errors;
use axum::response::{IntoResponse, Response};
use tower_http::catch_panic::CatchPanicLayer;

// todo: Is there an existing type we can use for CatchPanicLayer's type param instead of defining our own?
pub fn custom_catch_panic_layer() -> CatchPanicLayer<fn(Box<dyn std::any::Any + Send>) -> Response>
{
    CatchPanicLayer::custom(custom_panic_handler)
}

/// Handler function for the [`CatchPanicLayer`] middleware.
#[allow(clippy::needless_pass_by_value)]
pub fn custom_panic_handler(
    err: Box<dyn std::any::Any + Send + 'static>,
) -> axum::response::Response {
    let err = err.downcast_ref::<String>().map_or_else(
        || err.downcast_ref::<&str>().map_or("no error details", |s| s),
        |s| s.as_str(),
    );

    tracing::error!(err.msg = err, "server_panic");

    errors::Error::InternalServerError.into_response()
}
