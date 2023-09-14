use salvo::prelude::*;
use salvo::serve_static::StaticDir;

use crate::common::CONFIG;

use super::auth::{get_auth_hoop, HoopAuth};

pub trait ImageRoute {
    fn with_image_routes() -> Self;
    fn push_image_routes(self) -> Self;
}

impl ImageRoute for Router {
    fn with_image_routes() -> Self {
        Router::new().push_image_routes()
    }
    fn push_image_routes(self) -> Self {
        let path = CONFIG.image_url.clone() + "<*+img_path>";

        self.hoop_auth()
            .push(Router::with_path(path).get(StaticDir::new(CONFIG.image_dir.clone())))
    }
}
