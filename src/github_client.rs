use chrono::{DateTime, Local};
use reqwest::blocking::{Client, Response};
use reqwest::Error;

use crate::{GithubNotification, HasHtmlUrl, NotificationWithUrl};

/// https://developer.github.com/v3/activity/notifications/
const GITHUB_API_URL: &str = "https://api.github.com/notifications";

/// A client to get messages from the github notification API
///
/// # Arguments
///
/// * `client` - The reqwests client that gets from the github API
/// * `username` - A github username associated with the token
/// * `token` - A github personal access token with notification, repo, and non-admin read scopes
pub struct GithubClient {
    client: Client,
    username: String,
    token: String,
}

impl GithubClient {
    pub fn new(username: String, token: String) -> Self {
        Self {
            client: Client::new(),
            username,
            token,
        }
    }

    /// Gets all notifications since a timestamp even if they've been read
    fn get_notifications(&self, since: DateTime<Local>) -> Result<Response, Error> {
        self.client
            .get(GITHUB_API_URL)
            .basic_auth(&self.username, Some(&self.token))
            .query(&[("since", since.to_rfc3339()), ("all", String::from("true"))])
            .send()
    }

    /// Gets a response from an API URL
    fn get(&self, api_url: &str) -> Result<Response, Error> {
        self.client
            .get(api_url)
            .basic_auth(&self.username, Some(&self.token))
            .send()
    }

    /// Gets an HTML URL from a notification's API URL and bundles them together
    fn get_html_url(&self, notification: GithubNotification) -> Result<NotificationWithUrl, Error> {
        let response = self.get(&notification.subject.url)?;
        let url: HasHtmlUrl = response.json::<HasHtmlUrl>()?;
        Ok(NotificationWithUrl::new(notification, url))
    }

    /// Gets all notifications since a timestamp and bundles them with a human-usable HTML URL
    pub fn fetch_notifications(
        &self,
        since: DateTime<Local>,
    ) -> Result<Vec<NotificationWithUrl>, Error> {
        let response = self.get_notifications(since)?;
        let notifications: Vec<GithubNotification> = response.json::<Vec<GithubNotification>>()?;
        notifications
            .into_iter()
            .map(|notification| self.get_html_url(notification))
            .collect()
    }
}
