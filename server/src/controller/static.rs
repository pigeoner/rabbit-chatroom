use salvo::prelude::*;
use salvo::serve_static::StaticDir;

use crate::common::CONFIG;

use super::auth::HoopAuth;

pub trait StaticRoute {
    fn with_static_routes() -> Self;
    fn push_static_routes(self) -> Self;
}

impl StaticRoute for Router {
    fn with_static_routes() -> Self {
        Router::new().push_static_routes()
    }
    fn push_static_routes(self) -> Self {
        let path = CONFIG.static_url.clone() + "/<*+>";

        self.push(
            Router::with_path(path)
                .hoop_auth()
                .get(StaticDir::new(CONFIG.static_dir.clone())),
        )
    }
}
