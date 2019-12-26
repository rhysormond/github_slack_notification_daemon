use async_trait::async_trait;
use chrono::{DateTime, Local};
use reqwest::Error;
use reqwest::{Client, Response};

use crate::{GithubNotification, HasHtmlUrl, NotificationWithUrl};
use futures::future::join_all;

/// https://developer.github.com/v3/activity/notifications/
const GITHUB_API_URL: &str = "https://api.github.com/notifications";

/// A client to get messages from github—or perhaps one day another service's—notification API
///
/// # Arguments
///
/// * `client` - The `reqwests` client that gets from the github API
/// * `username` - A github username associated with the token
/// * `token` - A github personal access token with notification, repo, and non-admin read scopes
pub struct NotificationClient {
    client: Client,
    username: String,
    token: String,
}

#[async_trait]
pub trait GithubNotificationClient {
    /// Gets all notifications since a timestamp and bundles them with a human-usable HTML URL.
    async fn get_humanized_notifications(
        &self,
        since: DateTime<Local>,
    ) -> Result<Vec<NotificationWithUrl>, Error>;
}

impl NotificationClient {
    pub fn new(username: String, token: String) -> Self {
        Self {
            client: Client::new(),
            username,
            token,
        }
    }

    /// Gets all notifications since a timestamp, even if they've been read.
    async fn get_notifications(
        &self,
        since: DateTime<Local>,
    ) -> Result<Vec<GithubNotification>, Error> {
        self.client
            .get(GITHUB_API_URL)
            .basic_auth(&self.username, Some(&self.token))
            .query(&[("since", since.to_rfc3339()), ("all", String::from("true"))])
            .send()
            .await?
            .json::<Vec<GithubNotification>>()
            .await
    }

    /// Gets a response from an API URL
    async fn get_api(&self, api_url: &str) -> Result<Response, Error> {
        self.client
            .get(api_url)
            .basic_auth(&self.username, Some(&self.token))
            .send()
            .await
    }

    /// Gets an HTML URL from a notification's API URL and bundles them together
    async fn get_html_url(
        &self,
        notification: GithubNotification,
    ) -> Result<NotificationWithUrl, Error> {
        let response = self.get_api(&notification.subject.url).await?;
        let url: HasHtmlUrl = response.json::<HasHtmlUrl>().await?;
        Ok(NotificationWithUrl::new(notification, url))
    }
}

#[async_trait]
impl GithubNotificationClient for NotificationClient {
    async fn get_humanized_notifications(
        &self,
        since: DateTime<Local>,
    ) -> Result<Vec<NotificationWithUrl>, Error> {
        let notifications = self.get_notifications(since).await?;
        join_all(
            notifications
                .into_iter()
                .map(|notification| self.get_html_url(notification)),
        )
        .await
        .into_iter()
        .collect()
    }
}
