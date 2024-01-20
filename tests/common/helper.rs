use std::sync::Arc;

use togetherness::*;

enum Url {
    ApiPlayer,
    ApiTimer,
    ApiAction,
    ApiGold,
    ApiStatus,
    AdminReset,
    AdminSetTimer,
    AdminStartTimer,
    AdminStopTimer,
    AdminRepeatedActions,
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
            Url::AdminReset => self.root.join("/api/admin/reset").unwrap(),
            Url::AdminSetTimer => self.root.join("/api/admin/duration").unwrap(),
            Url::AdminStartTimer => self.root.join("/api/admin/start").unwrap(),
            Url::AdminStopTimer => self.root.join("/api/admin/stop").unwrap(),
            Url::AdminRepeatedActions => self.root.join("/api/admin/repeated_actions").unwrap(),
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
        let url = format!("{}/{}", self.make_url(Url::ApiGold), id.to_string());
        self.client
            .get(url)
            .send()
            .await?
            .json::<types::PlayerResponse>()
            .await
    }

    pub async fn post_player(
        &self,
        player: model::Player,
    ) -> Result<types::ActionResponse, reqwest::Error> {
        let player = types::PostPlayerRequest {
            id: player.id,
            name: player.name,
            avatar_id: player.avatar_id,
            faction_id: player.faction_id,
        };
        self.client
            .post(self.make_url(Url::ApiPlayer))
            .json(&player)
            .send()
            .await?
            .json::<types::ActionResponse>()
            .await
    }

    pub async fn post_action(
        &self,
        action: model::Action,
    ) -> Result<types::ActionResponse, reqwest::Error> {
        let action = types::PostActionRequest {
            action_id: action.action.into(),
            subject_id: action.subject_id,
            object_id: action.object_id,
        };
        self.client
            .post(self.make_url(Url::ApiAction))
            .json(&action)
            .send()
            .await?
            .json::<types::ActionResponse>()
            .await
    }

    pub async fn get_gold(
        &self,
        id: model::PlayerId,
    ) -> Result<types::GoldResponse, reqwest::Error> {
        let url = format!("{}/{}", self.make_url(Url::ApiGold), id.to_string());
        self.client
            .get(&url)
            .send()
            .await?
            .json::<types::GoldResponse>()
            .await
    }

    pub async fn reset_game(&self, repeated_actions_allowed: bool) {
        let stop_timer = self
            .client
            .post(self.make_url(Url::AdminStopTimer))
            .send()
            .await
            .unwrap()
            .json::<types::DefaultResponse>()
            .await
            .unwrap();
        assert!(matches!(stop_timer.error, types::Error::None));
        let reset = self
            .client
            .post(self.make_url(Url::AdminReset))
            .send()
            .await
            .unwrap()
            .json::<types::DefaultResponse>()
            .await
            .unwrap();
        assert!(matches!(reset.error, types::Error::None));
        let set_timer = self
            .client
            .post(self.make_url(Url::AdminSetTimer))
            .json(&50) // minutes
            .send()
            .await
            .unwrap()
            .json::<types::DefaultResponse>()
            .await
            .unwrap();
        assert!(matches!(set_timer.error, types::Error::None));
        let start_timer = self
            .client
            .post(self.make_url(Url::AdminStartTimer))
            .send()
            .await
            .unwrap()
            .json::<types::DefaultResponse>()
            .await
            .unwrap();
        assert!(matches!(start_timer.error, types::Error::None));
        if repeated_actions_allowed {
            let repeated_actions = self
                .client
                .post(self.make_url(Url::AdminRepeatedActions))
                .json(&repeated_actions_allowed)
                .send()
                .await
                .unwrap()
                .json::<types::DefaultResponse>()
                .await
                .unwrap();
            assert!(matches!(repeated_actions.error, types::Error::None));
        }
    }
}
