extern crate chrono;
extern crate reqwest;

use std::{env, thread, time};

use chrono::{DateTime, Local};
use reqwest::Error;

use crate::github_client::GithubClient;
use crate::github_notification::NotificationWithUrl;
use crate::slack_client::SlackClient;
use crate::slack_message::SlackMessage;

mod github_client;
mod github_notification;
mod slack_client;
mod slack_message;

const POLLING_FREQUENCY: time::Duration = time::Duration::from_secs(30);

fn main() {
    let github_username: String = env::var("GITHUB_USERNAME").expect("No GITHUB_USERNAME env var.");
    let github_token: String = env::var("GITHUB_TOKEN").expect("No GITHUB_TOKEN env var.");
    let slack_hook: String = env::var("SLACK_HOOK").expect("No SLACK_HOOK env var.");

    let github: GithubClient = GithubClient::new(github_username, github_token);
    let slack: SlackClient = SlackClient::new(slack_hook);

    let mut last_fetch_time: DateTime<Local> = Local::now();

    let initialization_message: SlackMessage =
        SlackMessage::new(format!("Initialized at {:?}", last_fetch_time));
    slack.post(&initialization_message).unwrap();

    loop {
        let time_before_fetch: DateTime<Local> = Local::now();

        let maybe_notifications: Result<Vec<NotificationWithUrl>, Error> =
            github.fetch_notifications(last_fetch_time);

        let notifications: Vec<NotificationWithUrl> = match maybe_notifications {
            Ok(notifications) => {
                println!("Got notifications from github: {:?}.", notifications);
                last_fetch_time = time_before_fetch;
                println!("Setting last_fetch_time to {:?}", last_fetch_time);
                notifications
            }
            Err(error) => {
                println!("Failed to get GitHub notifications: {:?}.", error);
                Vec::new()
            }
        };

        for notification in notifications {
            let message: SlackMessage = SlackMessage::from_notification(&notification);
            slack.post(&message).unwrap();
        }

        thread::sleep(POLLING_FREQUENCY);
    }
}
