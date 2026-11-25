use std::time::{Duration, Instant};
use std::io::{self};
use ureq::Agent;

pub fn send_msg(target: String, method: String, duration: Duration) -> (i64, i64) {
    let agent: Agent = Agent::config_builder().build().into();
    let start = Instant::now();
    let mut ok: i64 = 0;
    let mut err: i64 = 0;
    let mut sink = io::sink();

    match method.as_str() {
        "GET" => {
            while start.elapsed() < duration {
                match agent.get(target.as_str()).call() {
                    Ok(mut resp) => {
                        let _ = io::copy(&mut resp.body_mut().as_reader(), &mut sink);
                        ok += 1;
                    }
                    Err(_) => err += 1,
                }
            }
        }
        "POST" => {
            while start.elapsed() < duration {
                match agent.post(target.as_str()).send_empty() {
                    Ok(mut resp) => {
                        let _ = io::copy(&mut resp.body_mut().as_reader(), &mut sink);
                        ok += 1;
                    }
                    Err(_) => err += 1,
                }
            }
        }
        _ => {}
    }

    (ok, err)
}
