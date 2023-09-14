mod user;
mod utils;

use salvo::prelude::*;

use self::user::UserController;
use crate::common::CONFIG;

pub async fn serve() {
    let acceptor = TcpListener::new(&CONFIG.addr_listen).bind().await;

    let router = Router::with_path("chatroom/v1")
        .get(hello)
        .push_user_routes();

    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn hello(res: &mut Response) {
    res.render("hello world");
}
