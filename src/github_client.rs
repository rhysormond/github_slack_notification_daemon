extern crate chrono;
extern crate reqwest;

use crate::github_notification::{GithubNotification, HasHtmlUrl, NotificationWithUrl};
use chrono::{DateTime, Local};
use reqwest::blocking::{Client, Response};
use reqwest::Error;

/// https://developer.github.com/v3/activity/notifications/
const GITHUB_API_URL: &str = "https://api.github.com/notifications";

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

    fn get_notifications(&self, since: DateTime<Local>) -> Result<Response, Error> {
        self.client
            .get(GITHUB_API_URL)
            .basic_auth(&self.username, Some(&self.token))
            .query(&[("since", since.to_rfc3339()), ("all", String::from("true"))])
            .send()
    }

    fn get(&self, api_url: &str) -> Result<Response, Error> {
        self.client
            .get(api_url)
            .basic_auth(&self.username, Some(&self.token))
            .send()
    }

    fn get_html_url(&self, notification: GithubNotification) -> Result<NotificationWithUrl, Error> {
        let response: Response = self.get(&notification.subject.url)?;
        let url: HasHtmlUrl = response.json::<HasHtmlUrl>()?;
        Ok(NotificationWithUrl::new(notification, url))
    }

    pub fn fetch_notifications(&self, since: DateTime<Local>) -> Result<Vec<NotificationWithUrl>, Error> {
        let response: Response = self.get_notifications(since)?;
        let notifications: Vec<GithubNotification> = response.json::<Vec<GithubNotification>>()?;
        notifications
            .into_iter()
            .map(|notification|
                self.get_html_url(notification)
            ).collect()
    }
}
