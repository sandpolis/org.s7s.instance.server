pub mod server {

    use actix_session::Session;
    use actix_web::{
        http::{header::ContentType, StatusCode},
        HttpRequest, HttpResponse, Result,
    };

    #[actix_web::post("/v1/server/{iid}/session")]
    async fn new_session(rq: HttpRequest, session: Session) -> Result<HttpResponse> {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::octet_stream())
            .body(vec![]))
    }

    #[actix_web::delete("/v1/server/{iid}/session")]
    async fn delete_session(rq: HttpRequest, session: Session) -> Result<HttpResponse> {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::octet_stream())
            .body(vec![]))
    }

    #[actix_web::get("/v1/server/banner")]
    async fn banner(rq: HttpRequest) -> Result<HttpResponse> {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::octet_stream())
            .body(vec![]))
    }
}

pub mod agent {

    use actix_web::{
        http::{header::ContentType, StatusCode},
        web, HttpRequest, HttpResponse, Result,
    };

    #[actix_web::get("/v1/agent/{iid}/reboot")]
    async fn reboot(rq: HttpRequest, iid: web::Path<String>) -> Result<HttpResponse> {
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type(ContentType::octet_stream())
            .body(vec![]))
    }
}
