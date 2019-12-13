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
            "{kind} - {title}\n*{reason}*\n{url}",
            kind = notification.notification.subject.kind,
            title = notification.notification.subject.title,
            reason = notification.notification.reason,
            url = notification.url,
        );
        Self::new(message)
    }
}
