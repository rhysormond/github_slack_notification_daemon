use serde::Serialize;

use crate::NotificationWithUrl;

/// A Slack message formatted for their incoming webhook API
///
/// # Arguments
///
/// * `text` - The text body of the post; supports markdown formatting
/// * `icon_emoji` - A slack-formatted emoji name that should be used in place of an avatar
/// * `username` - The username associated with the post
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

    /// Formats a github notification into the text field of a slack message
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
