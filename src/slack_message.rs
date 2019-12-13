extern crate serde;

use serde::Serialize;

use crate::github_notification::{GithubNotification, HasHtmlUrl};

#[derive(Serialize, Debug)]
pub struct SlackMessage {
    text: String,
    icon_emoji: String,
    username: String,
}

impl SlackMessage {
    pub fn new(text: String) -> Self {
        SlackMessage {
            text,
            icon_emoji: ":chart_with_upwards_trend:".to_string(),
            username: "Github Notification Daemon".to_string(),
        }
    }

    pub fn from_github_notification(notification: &GithubNotification, body: HasHtmlUrl) -> Self {
        let message: String = format!(
            "{kind} from GitHub at {url}",
            kind = notification.subject.kind,
            url = body.html_url,
        );
        Self::new(message)
    }
}
