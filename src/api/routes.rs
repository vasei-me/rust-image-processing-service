use axum::{
    routing::{get, post},
    Router,
};
use crate::api::handlers;
use crate::application::user_service::UserService;
use crate::application::image_service::ImageService;
use crate::domain::user_repository::UserRepository;
use crate::domain::image::ImageRepository;
use crate::core::jwt::JwtService;

pub fn create_router<UR, IR>(
    user_service: UserService<UR>,
    image_service: ImageService<IR>,
    jwt_service: JwtService,
) -> Router
where
    UR: UserRepository + Clone + Send + Sync + 'static,
    IR: ImageRepository + Clone + Send + Sync + 'static,
{
    let auth_router = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .with_state((user_service, jwt_service));

    // Routes بدون authentication برای تست
    let image_router = Router::new()
        .route("/images", post(handlers::upload_image_simple).get(handlers::list_images_simple))
        .route("/images/:id", get(handlers::get_image_simple))
        .route("/images/:id/transform", post(handlers::transform_image_simple))
        .with_state(image_service);

    Router::new()
        .nest("/auth", auth_router)
        .nest("/api", image_router)
}