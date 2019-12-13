use serde::Serialize;

use crate::NotificationWithUrl;

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

    pub fn from_notification(notification: &NotificationWithUrl) -> Self {
        let message = format!(
            "{kind} from GitHub at {url}",
            kind = notification.subject.kind,
            url = notification.url,
        );
        Self::new(message)
    }
}
