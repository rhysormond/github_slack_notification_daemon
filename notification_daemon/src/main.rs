extern crate github_notifications;

use std::{env, thread, time};

use chrono::{Duration, Local};

use github_notifications::*;

use log::{debug, error, info};

const POLLING_FREQUENCY: time::Duration = time::Duration::from_secs(30);

fn main() {
    env_logger::init();
    let github_username = env::var("GITHUB_USERNAME").expect("No GITHUB_USERNAME env var.");
    let github_token = env::var("GITHUB_TOKEN").expect("No GITHUB_TOKEN env var.");
    let slack_hook = env::var("SLACK_HOOK").expect("No SLACK_HOOK env var.");

    let github = GithubClient::new(github_username, github_token);
    let slack = SlackClient::new(slack_hook);

    let prefetch_time: Duration = Duration::hours(1);
    let mut last_fetch_time = Local::now() - prefetch_time;

    let initialization_message = format!(
        "Initializing at {:?} and fetching messages from the last {:?}",
        last_fetch_time, prefetch_time,
    );
    slack.post(initialization_message).unwrap();

    loop {
        let time_before_fetch = Local::now();

        let maybe_notifications = github.fetch_notifications(last_fetch_time);

        let notifications = match maybe_notifications {
            Ok(notifications) => {
                info!("Got notifications from github: {:?}.", notifications);
                last_fetch_time = time_before_fetch;
                debug!("Setting last_fetch_time to {:?}", last_fetch_time);
                notifications
            }
            Err(error) => {
                let msg = format!("Failed to get GitHub notifications: {:?}.", error);
                error!("Failed to get GitHub notifications: {:?}.", error);
                slack.post(msg).unwrap();
                Vec::new()
            }
        };

        for notification in notifications {
            slack.post(notification).unwrap();
        }

        thread::sleep(POLLING_FREQUENCY);
    }
}