use crate::api::v1::util::rs_body;
use crate::user::User;
use crate::user::UserBuilder;
use crate::ServerContext;
use actix_session::Session;
use actix_web::web;
use actix_web::{http::StatusCode, HttpRequest, HttpResponse, Result};
use core_protocol::core::protocol::{GetUserResponse, PostUserRequest};
use prost::Message;

#[actix_web::post("/v1/server/{iid}/users")]
async fn add_user(
    session: Session,
    body: web::Bytes,
    context: web::Data<ServerContext>,
) -> Result<HttpResponse> {
    match PostUserRequest::decode(body) {
        Ok(rs) => {
            let users = context.users.read().unwrap();

            if let Some(_) = users.iter().find(|&user| user.username == rs.username) {
                Ok(HttpResponse::new(StatusCode::BAD_REQUEST))
            } else {
                UserBuilder::new(&rs.username).build().unwrap();
            }
        }
        Err(_) => Ok(HttpResponse::new(StatusCode::BAD_REQUEST)),
    }
}

#[actix_web::get("/v1/server/{iid}/users")]
async fn list_users(
    rq: HttpRequest,
    body: web::Bytes,
    session: Session,
    context: web::Data<ServerContext>,
    iid: web::Path<String>,
) -> Result<HttpResponse> {
    /*match check_session(session, &context.db).await {
        Ok(user) => {
        },
        Err(_) =>
    }*/

    if *iid != context.iid {
        return forward();
    }

    let users = context.users.read().unwrap();

    check_session!(session, users);

    rs_body!(GetUserResponse {
        user: users
            .iter()
            .map(|user| {
                core_protocol::core::protocol::get_user_response::User {
                    username: user.username.clone(),
                    email: user.email.clone(),
                    phone: user.phone.clone(),
                    expiration: user.expiration,
                }
            })
            .collect(),
    })
}
