use axum::{Router};

use crate::{
    enums::routes::RoutePath,
    features::{
        auth::routes::get_routes as get_auth_routes,
        users::routes::get_routes as get_user_routes,
    },
};

pub fn create_router() -> Router {
    let auth_routes = get_auth_routes();
    let user_routes = get_user_routes(); 

    let api_routes = Router::new()
    .nest(RoutePath::AUTH.get_path(), auth_routes)
    .nest(RoutePath::USERS.get_path(), user_routes);

    Router::new().nest("/api", api_routes)  
}
