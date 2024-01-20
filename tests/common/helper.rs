use std::sync::Arc;

use togetherness::*;

enum Url {
    ApiPlayer,
    ApiTimer,
    ApiAction,
    ApiGold,
    ApiStatus,
}

pub struct Helper {
    root: reqwest::Url,
    client: Arc<reqwest::Client>,
}

impl Helper {
    pub fn new(root: reqwest::Url, client: Arc<reqwest::Client>) -> Self {
        Self {
            root,
            client: client,
        }
    }

    fn make_url(&self, url: Url) -> reqwest::Url {
        match url {
            Url::ApiPlayer => self.root.join("/api/player").unwrap(),
            Url::ApiTimer => self.root.join("/api/timer").unwrap(),
            Url::ApiAction => self.root.join("/api/action").unwrap(),
            Url::ApiGold => self.root.join("/api/gold").unwrap(),
            Url::ApiStatus => self.root.join("/api/status").unwrap(),
        }
    }

    pub async fn get_timer(&self) -> Result<types::TimerResponse, reqwest::Error> {
        self.client
            .get(self.make_url(Url::ApiTimer))
            .send()
            .await?
            .json::<types::TimerResponse>()
            .await
    }

    pub async fn get_player(
        &self,
        id: model::PlayerId,
    ) -> Result<types::PlayerResponse, reqwest::Error> {
        self.client
            .get(self.make_url(Url::ApiPlayer).join(id.as_str()).unwrap())
            .send()
            .await?
            .json::<types::PlayerResponse>()
            .await
    }
}
