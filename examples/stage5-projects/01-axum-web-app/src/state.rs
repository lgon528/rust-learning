use crate::auth::JwtService;
use crate::services::{UserService, PostService};

// Application state
#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub post_service: PostService,
    pub jwt_service: JwtService,
}
