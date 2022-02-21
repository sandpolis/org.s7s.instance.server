use actix_web::{HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Connection {
    // The remote instance ID
    pub iid: String,

    // The remote host address and port
    pub address: String,

    #[serde(skip)]
    pub client: reqwest::Client,
}

impl Connection {
    pub fn new() -> anyhow::Result<Self> {
        let ident = reqwest::Identity::from_pem()?;

        Ok(Connection {
            client: reqwest::Client::builder()
                .cookie_store(true)
                .http2_prior_knowledge()
                .identity(ident)
                .build()
                .unwrap(),
        })
    }

    pub async fn forward(&self, rq: &HttpRequest) -> actix_web::Result<HttpResponse> {}
}
