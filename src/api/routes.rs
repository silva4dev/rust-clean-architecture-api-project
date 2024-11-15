use warp::Filter;
use crate::infrastructure::controllers::http::{add_account_controller::add_account_controller, auth_account_controller::auth_account_controller};

pub fn account_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let register_route = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path("register"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add_account_controller);

    let login_route = warp::path("api")
        .and(warp::path("v1"))
        .and(warp::path("login"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(auth_account_controller);

    register_route.or(login_route)
}

pub fn routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    account_routes()
        .with(warp::cors().allow_any_origin()) 
}