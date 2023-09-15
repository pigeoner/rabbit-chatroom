/// ## 添加controller的cookbook
/// 1. 在controller模块下新建模块，例如user
/// 2. 为user模块添加handler
/// 3. 在handler中，所有遇到的错误类型，为其实现trait self::utils::ErrorRender
/// 4. 在handler中，使用render_error!宏，归一化错误类型的逻辑处理
/// 5. 为user模块添加UserRoute特征，包含push_user_routes方法，为Router实现UserRoute特征
/// 6. 在controller模块中serve函数内，调用push_user_routes方法
mod auth;
mod r#static;
mod user;

mod utils;

use salvo::prelude::*;

use self::{r#static::StaticRoute, user::UserRoute};
use crate::common::CONFIG;

pub async fn serve() {
    let acceptor = TcpListener::new(&CONFIG.addr_listen).bind().await;

    let router = Router::with_path("chatroom/v1")
        .push_user_routes()
        .push_static_routes();

    Server::new(acceptor).serve(router).await;
}
