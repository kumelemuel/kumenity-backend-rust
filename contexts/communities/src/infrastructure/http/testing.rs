//! Test support for the HTTP adapter, without an async runtime: the use cases
//! are still synchronous (RS-3), so every future here is ready on first poll.

use axum::response::Response;
use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

/// Body of D7: all the client sees of a `Fault`.
pub const D7_BODY: &str = r#"{"code":"INTERNAL_ERROR","message":"We couldn't complete your request right now. Please try again."}"#;

/// Polls `future` once. Panics if it is still pending.
pub fn ready<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let Poll::Ready(output) = future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    else {
        panic!("invariante: sin E/S real, el future está listo en el primer poll");
    };
    output
}

pub fn body_of(response: Response) -> String {
    let bytes = ready(axum::body::to_bytes(response.into_body(), usize::MAX))
        .expect("invariante: el cuerpo de un Json se lee entero");
    String::from_utf8(bytes.to_vec()).expect("invariante: un Json es UTF-8")
}
