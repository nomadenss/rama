use crate::service::{Context, Layer, Service};
use std::fmt;

/// Service returned by the [`map_response`] combinator.
///
/// [`map_response`]: crate::service::ServiceBuilder::map_response
pub struct MapResponse<S, F> {
    inner: S,
    f: F,
}

impl<S, F> Clone for MapResponse<S, F>
where
    S: Clone,
    F: Clone,
{
    fn clone(&self) -> Self {
        MapResponse {
            inner: self.inner.clone(),
            f: self.f.clone(),
        }
    }
}

impl<S, F> fmt::Debug for MapResponse<S, F>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapResponse")
            .field("inner", &self.inner)
            .field("f", &format_args!("{}", std::any::type_name::<F>()))
            .finish()
    }
}

/// A [`Layer`] that produces a [`MapResponse`] service.
///
/// [`Layer`]: crate::service::Layer
pub struct MapResponseLayer<F> {
    f: F,
}

impl<F> Clone for MapResponseLayer<F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        MapResponseLayer { f: self.f.clone() }
    }
}

impl<F> fmt::Debug for MapResponseLayer<F>
where
    F: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapResponseLayer")
            .field("f", &format_args!("{}", std::any::type_name::<F>()))
            .finish()
    }
}

impl<S, F> MapResponse<S, F> {
    /// Creates a new `MapResponse` service.
    pub fn new(inner: S, f: F) -> Self {
        MapResponse { f, inner }
    }
}

impl<S, F, State, Request, Response> Service<State, Request> for MapResponse<S, F>
where
    S: Service<State, Request>,
    F: Fn(S::Response) -> Response + Send + Sync + 'static,
    State: Send + Sync + 'static,
    Request: Send + 'static,
    Response: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;

    async fn serve(
        &self,
        ctx: Context<State>,
        req: Request,
    ) -> Result<Self::Response, Self::Error> {
        match self.inner.serve(ctx, req).await {
            Ok(resp) => Ok((self.f)(resp)),
            Err(err) => Err(err),
        }
    }
}

impl<F> MapResponseLayer<F> {
    /// Creates a new [`MapResponseLayer`] layer.
    pub fn new(f: F) -> Self {
        MapResponseLayer { f }
    }
}

impl<S, F> Layer<S> for MapResponseLayer<F>
where
    F: Clone,
{
    type Service = MapResponse<S, F>;

    fn layer(&self, inner: S) -> Self::Service {
        MapResponse {
            f: self.f.clone(),
            inner,
        }
    }
}
