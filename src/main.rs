#![crate_name = "syncazoom"]

use argh::FromArgs;
use chrono::offset::Utc;
use chrono::DateTime;
use cron::Schedule;
use job_scheduler::{Job, JobScheduler};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::error;
use std::fmt;
use std::fs;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use toml::de::Error;

mod database;
mod helpers;
mod zoom;

use crate::database::*;
use crate::helpers::*;
use crate::zoom::*;

/// This is the main application loop 😀
///
/// The program will fetch Zoom responses on a set interval and respect already running processes
/// The application will send Slack updates to a user set webhook.
///```
/// fn main() -> Result<(), Error> {
///     // Only run if we get a config file
///     let up: GoUp = argh::from_env();
///
///     // parse config and make vars
///     let toml_content: String = fs::read_to_string(up.config).unwrap();
///     let package_info: toml::Value = toml::from_str(&toml_content)?;
///     let key = package_info["creds"]["key"].as_str().unwrap().to_string();
///     let secr = package_info["creds"]["secret"]
///         .as_str()
///         .unwrap()
///         .to_string();
///     let seconds_between_calls = package_info["settings"]["seconds_between_calls"]
///         .as_integer()
///         .unwrap();
///     let cron_interval = package_info["settings"]["cron_interval"]
///         .as_str()
///         .unwrap()
///         .to_string();
///
///     let slack_webhook = package_info["slack"]["webhook"]
///         .as_str()
///         .unwrap()
///         .to_string();
///
///     send_slack_message(&slack_webhook, "🟢 Program start");
///
///     // we start a scheduler and way to communicate with the background thread
///     let mut sched = JobScheduler::new();
///     let mut count = 0;
///     let mut is_running = false;
///     let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
///
///     let schedule = Schedule::from_str(&cron_interval).unwrap();
///     println!("Upcoming fire times:");
///     for datetime in schedule.upcoming(Utc).take(10) {
///         println!("-> {}", datetime);
///     }
///
///     sched.add(Job::new(cron_interval.parse().unwrap(), move || {
///         let thread_tx = tx.clone();
///
///         let _key = key.clone();
///         let _secr = secr.clone();
///         // if its not running we should run it
///         if !is_running {
///             is_running = true;
///
///             println!("\n");
///             print_time();
///             println!("Start run");
///             send_slack_message(&slack_webhook, "🏃‍♀️ Start process");
///
///             thread::spawn(move || {
///                 execute(
///                     _key.to_string(),
///                     _secr.to_string(),
///                     seconds_between_calls as u64,
///                 );
///                 thread_tx.send(1).unwrap();
///             });
///         }
///
///         match rx.try_recv() {
///             Ok(msg) => {
///                 println!("End Run");
///                 send_slack_message(&slack_webhook, "✅ Complete process");
///                 print_time();
///                 is_running = false
///             }
///             Err(_) => (),
///         }
///
///         count += 1;
///     }));
///
///     loop {
///         sched.tick();
///
///         std::thread::sleep(Duration::from_millis(500));
///     }
///     // #[warn(unreachable_code)]
///     Ok(())
/// }
/// ```
fn main() -> Result<(), Error> {
    // Only run if we get a config file
    let up: GoUp = argh::from_env();

    // parse config and make vars
    let toml_content: String = fs::read_to_string(up.config).unwrap();
    let package_info: toml::Value = toml::from_str(&toml_content)?;

    // required
    let key = package_info["creds"]["key"].as_str().unwrap().to_string();

    let secr = package_info["creds"]["secret"]
        .as_str()
        .unwrap()
        .to_string();

    let seconds_between_calls = package_info["settings"]["seconds_between_calls"]
        .as_integer()
        .unwrap();

    // purely optional
    let slack_webhook = package_info["slack"]["webhook"]
        .as_str()
        .unwrap()
        .to_string();

    // conditional
    let single_run = package_info["settings"]["single_run"].as_bool().unwrap();

    let cron_interval = package_info["settings"]["cron_interval"]
        .as_str()
        .unwrap()
        .to_string();

    send_slack_message(&slack_webhook, "🤽‍♀️ Program start");

    match single_run {
        true => {
            println!("Stateless Runtime");

            send_slack_message(&slack_webhook, " - 🏃‍♀️ Stating scraper");
            let _key = key.clone();
            let _secr = secr.clone();
            execute(
                _key.to_string(),
                _secr.to_string(),
                seconds_between_calls as u64,
            );
            send_slack_message(&slack_webhook, " - ✅ Complete process");
        }
        false => {
            println!("Runtime has state");

            // we start a scheduler and way to communicate with the background thread
            let mut sched = JobScheduler::new();
            let mut count = 0;
            let mut is_running = false;
            let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

            let schedule = Schedule::from_str(&cron_interval).unwrap();
            println!("Upcoming fire times:");
            for datetime in schedule.upcoming(Utc).take(10) {
                println!("-> {}", datetime);
            }

            sched.add(Job::new(cron_interval.parse().unwrap(), move || {
                let thread_tx = tx.clone();

                let _key = key.clone();
                let _secr = secr.clone();
                // if its not running we should run it
                if !is_running {
                    is_running = true;

                    println!("\n");
                    print_time();
                    println!("Start run");
                    send_slack_message(&slack_webhook, " - 🏃‍♀️ Start process");

                    let swh = slack_webhook.clone();
                    thread::spawn(move || {
                        execute(
                            _key.to_string(),
                            _secr.to_string(),
                            seconds_between_calls as u64,
                        );

                        println!("End Run");
                        send_slack_message(&swh, " - ✅ Complete process");
                        thread_tx.send(1).unwrap();
                    });
                }

                match rx.try_recv() {
                    Ok(_msg) => {
                        println!("Channel changes `is_running` state");
                        print_time();
                        is_running = false
                    }
                    Err(_) => (),
                }

                count += 1;
            }));

            loop {
                sched.tick();

                std::thread::sleep(Duration::from_millis(500));
            }
        }
    }

    // #[warn(unreachable_code)]
    Ok(())
}
