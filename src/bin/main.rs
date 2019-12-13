extern crate github_notifications;

use std::{env, thread, time};

use chrono::Local;

use github_notifications::*;

const POLLING_FREQUENCY: time::Duration = time::Duration::from_secs(30);

fn main() {
    let github_username = env::var("GITHUB_USERNAME").expect("No GITHUB_USERNAME env var.");
    let github_token = env::var("GITHUB_TOKEN").expect("No GITHUB_TOKEN env var.");
    let slack_hook = env::var("SLACK_HOOK").expect("No SLACK_HOOK env var.");

    let github = GithubClient::new(github_username, github_token);
    let slack = SlackClient::new(slack_hook);

    let mut last_fetch_time = Local::now();

    let initialization_message = SlackMessage::new(format!("Initialized at {:?}", last_fetch_time));
    slack.post(&initialization_message).unwrap();

    loop {
        let time_before_fetch = Local::now();

        let maybe_notifications = github.fetch_notifications(last_fetch_time);

        let notifications = match maybe_notifications {
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
            let message = SlackMessage::from_notification(&notification);
            slack.post(&message).unwrap();
        }

        thread::sleep(POLLING_FREQUENCY);
    }
}
