mod models;
mod repos;

use models::TokenClaims;
use salvo::prelude::*;
use repos::{
    public::{
        health_checker_handler,
        teachers
    },
    auth::{login, self},
};
use jsonwebtoken::{self, EncodingKey};
use salvo::http::{Method, StatusError};
use salvo::jwt_auth::QueryFinder;


const SECRET_KEY: &str = "MYSUPERSECRETKEY";

fn auth_handler() -> JwtAuth<TokenClaims> {
    JwtAuth::new(SECRET_KEY.to_owned())
        .finders(vec![
            Box::new(QueryFinder::new("jwt_token")),
        ])
        .response_error(true) // if true then current page not work.
}

fn router_creator() -> Router {
    Router::new()
        .get(health_checker_handler)
        .push(
            Router::with_path("auth")
            .push(
                Router::with_path("login")
                    .handle(login)
            )
            .push(
                Router::with_path("register")
                    .get(login)
            )
        )
        .push(
            Router::with_path("api")
                .push(
                    Router::with_path("public")
                        .push(
                            Router::with_path("teachers")
                                .hoop(auth_handler())
                                .get(teachers)
                        )
                )
        )
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router_creator()).await;
}
