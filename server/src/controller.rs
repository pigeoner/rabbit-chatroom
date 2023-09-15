mod auth;
mod image;
mod user;

mod utils;

use salvo::prelude::*;

use self::{image::ImageRoute, user::UserRoute};
use crate::common::CONFIG;

pub async fn serve() {
    let acceptor = TcpListener::new(&CONFIG.addr_listen).bind().await;

    let router = Router::with_path("chatroom/v1")
        .push_user_routes()
        .push_image_routes();

    Server::new(acceptor).serve(router).await;
}