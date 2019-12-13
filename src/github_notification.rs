use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Subject {
    pub title: String,
    pub url: String,
    latest_comment_url: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubNotification {
    id: String,
    pub subject: Subject,
    pub reason: String,
    unread: bool,
    updated_at: String,
    last_read_at: Option<String>,
    url: String,
}

/// This should eventually be parted into different things for PRs, Issues, etc.
#[derive(Deserialize, Debug)]
pub struct HasHtmlUrl {
    pub html_url: String,
}

#[derive(Debug)]
pub struct NotificationWithUrl {
    pub notification: GithubNotification,
    pub url: String,
}

impl NotificationWithUrl {
    pub fn new(notification: GithubNotification, url: HasHtmlUrl) -> Self {
        Self {
            notification,
            url: url.html_url,
        }
    }
}
