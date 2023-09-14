mod image;
mod user;
mod auth;

mod utils;

use salvo::prelude::*;

use self::{user::UserRoute, image::ImageRoute};
use crate::common::CONFIG;

pub async fn serve() {
    let acceptor = TcpListener::new(&CONFIG.addr_listen).bind().await;

    let router = Router::with_path("chatroom/v1")
        .push_user_routes()
        .push_image_routes();

    Server::new(acceptor).serve(router).await;
}