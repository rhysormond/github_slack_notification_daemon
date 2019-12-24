use serde::Deserialize;

/// The main body of a github notification API response
///
/// # Arguments
///
/// * `title` - The title of the subject of the notification (e.g. PR title)
/// * `url` - The API url for getting more information about the subject
/// * `kind` - The type of the subject (e.g. PR, Issue, etc.)
#[derive(Deserialize, Debug)]
pub struct Subject {
    pub title: String,
    pub url: String,
    latest_comment_url: String,
    #[serde(rename = "type")]
    pub kind: String,
}

/// The top level fields in a github notification API response
///
/// # Arguments
///
/// * `subject` - The body of the notification
/// * `reason` - The reason that the notification was sent
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

/// The HTML URL extracted from the response to a github get request to an API URL
///
/// NOTE: There many more fields in the response they depend on what api route we actually hit
#[derive(Deserialize, Debug)]
pub struct HasHtmlUrl {
    pub html_url: String,
}

/// A github notification bundled with an HTML url in addition to the original API one
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

impl std::convert::Into<String> for NotificationWithUrl {
    fn into(self) -> String {
        format!(
            "{kind} - {title}\n*{reason}*\n{url}",
            kind = self.notification.subject.kind,
            title = self.notification.subject.title,
            reason = self.notification.reason,
            url = self.url,
        )
    }
}
