use crate::{
    containers::app::AppContainer, middleware::auth::auth_middleware,
};
use axum::{middleware, Router};
use tower_http::trace::TraceLayer;

pub struct Server {}

impl Server {
    pub fn build(container: &AppContainer) -> Router {
        let iam_routes = container.iam.routes();
        let communities_routes = container.communities.routes();

        let public = Router::new()
            .merge(iam_routes.public())
            .merge(communities_routes.public());

        let protected = Router::new()
            .merge(iam_routes.protected())
            .merge(communities_routes.protected())
            .layer(middleware::from_fn_with_state(
                container.shared.jwt_service.clone(),
                auth_middleware,
            ));

        Router::new()
            .merge(public)
            .merge(protected)
            .layer(TraceLayer::new_for_http())
    }
}
