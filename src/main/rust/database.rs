use serde::{Deserialize, Serialize};
use anyhow::{Result, bail};

#[derive(Serialize)]
struct PostSession {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RsPostSession {
    ok: bool,
    name: String,
}

pub struct DbConnection {
	host: String,
    client: reqwest::Client,
}

impl DbConnection {
    pub async fn new(
        host: String,
        username: String,
        password: String,
    ) -> Result<DbConnection> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;

        let rs = client
            .post(host.clone() + "/_session")
            .json(&PostSession {
                username: username,
                password: password,
            })
            .send()
            .await?
            .json::<RsPostSession>()
            .await?;

        if ! rs.ok {
        	bail!("Account error");
        }

        Ok(DbConnection { host: host, client: client })
    }

    pub async fn create_db(&self, db: String) -> Result<()> {
    	self.client.put(self.host + "/" + db).await
    }
}
