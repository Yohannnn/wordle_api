use chrono::{Local, NaiveTime, TimeDelta};
use words::get_word;

mod filters;
mod handlers;
mod models;
mod words;

use std::{
    net::Ipv6Addr,
    sync::{Arc, RwLock},
};

#[tokio::main]
async fn main() {
    let answer = Arc::from(RwLock::new(words::get_word()));
    tokio::join!(
        async {
            warp::serve(filters::guess(answer.clone()))
                .run((Ipv6Addr::LOCALHOST, 3030))
                .await;
        },
        async {
            loop {
                let now = Local::now().naive_local();
                let next = now
                    .date()
                    .and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                    + TimeDelta::days(1);
                let duration = next.signed_duration_since(now);
                tokio::time::sleep(duration.to_std().unwrap()).await;

                let mut answer = answer.write().unwrap();
                *answer = get_word().to_string();
            }
        }
    );
}
