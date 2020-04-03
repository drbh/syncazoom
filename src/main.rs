use argh::FromArgs;
use chrono::offset::Utc;
use chrono::DateTime;
use job_scheduler::{Job, JobScheduler};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::error;
use std::fmt;
use std::fs;
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

fn print_time() {
    let start = SystemTime::now();
    let datetime: DateTime<Utc> = start.into();
    println!("{}", datetime.format("%d/%m/%Y %T"));
}

fn main() -> Result<(), Error> {
    // Only run if we get a config file
    let up: GoUp = argh::from_env();

    // parse config and make vars
    let toml_content: String = fs::read_to_string(up.config).unwrap();
    let package_info: toml::Value = toml::from_str(&toml_content)?;
    let key = package_info["creds"]["key"].as_str().unwrap().to_string();
    let secr = package_info["creds"]["secret"]
        .as_str()
        .unwrap()
        .to_string();
    let seconds_between_calls = package_info["settings"]["seconds_between_calls"]
        .as_integer()
        .unwrap();
    let cron_interval = package_info["settings"]["cron_interval"]
        .as_str()
        .unwrap()
        .to_string();

    //
    let mut sched = JobScheduler::new();
    let mut count = 0;
    let mut is_running = false;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

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

            thread::spawn(move || {
                execute(
                    _key.to_string(),
                    _secr.to_string(),
                    seconds_between_calls as u64,
                );
                thread_tx.send(1).unwrap();
            });
        }

        match rx.try_recv() {
            Ok(msg) => {
                println!("End Run");
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
    // #[warn(unreachable_code)]
    Ok(())
}
