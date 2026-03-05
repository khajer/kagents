use chrono::{Local, NaiveDate, NaiveDateTime, Timelike};
use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::info;

#[derive(Parser)]
#[command(name = "mbot")]
#[command(version = "0.1.0")]
#[command(about = "A scheduler bot", long_about = None)]
struct Cli {}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Task {
    pub completed: bool,
    pub date: NaiveDate,
    pub time: Option<String>,
    pub description: String,
}

impl Task {
    pub fn datetime(&self) -> Option<NaiveDateTime> {
        self.time.as_ref().and_then(|t| {
            let dt_str = format!("{} {}", self.date, t);
            NaiveDateTime::parse_from_str(&dt_str, "%Y-%m-%d %H:%M").ok()
        })
    }

    pub fn unique_key(&self) -> String {
        match &self.time {
            Some(t) => format!("{}-{}-{}", self.date, t, self.description),
            None => format!("{}-allday-{}", self.date, self.description),
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = if self.completed { "[x]" } else { "[ ]" };
        match &self.time {
            Some(t) => write!(f, "{} {} {} : {}", status, self.date, t, self.description),
            None => write!(f, "{} {} : {}", status, self.date, self.description),
        }
    }
}

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();
    
    tracing_subscriber::fmt::init();
    info!("mbot scheduler started");

    let reminded = Arc::new(RwLock::new(HashSet::<String>::new()));
    let mut ticker = interval(Duration::from_secs(60));

    loop {
        ticker.tick().await;

        let tasks = parse_tasks(&read_markdown_file("schedules/schedule.md"));
        let now = Local::now().naive_local();
        let today = now.date();
        let current_time = now.time();

        let mut reminded_guard = reminded.write().await;

        for task in tasks {
            if task.completed {
                continue;
            }

            let key = task.unique_key();

            if reminded_guard.contains(&key) {
                continue;
            }

            let should_remind = match task.datetime() {
                Some(task_dt) => {
                    let diff = (task_dt - now).num_seconds();
                    (0..60).contains(&diff)
                }
                None => {
                    task.date == today && current_time.hour() == 9 && current_time.minute() == 0
                }
            };

            if should_remind {
                info!(target: "reminder", "REMINDER: {} | Scheduled: {} {}",
                    task.description,
                    task.date,
                    task.time.as_deref().unwrap_or("all-day")
                );
                reminded_guard.insert(key);
            }
        }
    }
}

pub fn read_markdown_file(path: &str) -> String {
    fs::read_to_string(path).expect("Failed to read markdown file")
}

pub fn parse_tasks(content: &str) -> Vec<Task> {
    let mut tasks = Vec::new();
    let re = Regex::new(r"^- \[([ xX])\]\s*(\d{4}-\d{2}-\d{2})(?:\s+(\d{2}:\d{2}))?\s*:\s*(.+)$")
        .unwrap();

    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            let completed = caps[1].to_lowercase() == "x";
            let date = NaiveDate::parse_from_str(&caps[2], "%Y-%m-%d").ok();
            let time = caps.get(3).map(|m| m.as_str().to_string());
            let description = caps[4].trim().to_string();

            if let Some(date) = date {
                tasks.push(Task {
                    completed,
                    date,
                    time,
                    description,
                });
            }
        }
    }

    tasks
}
