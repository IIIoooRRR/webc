
pub mod send;
use crate::send::send_msg;
use std::env;
use std::sync::Arc;
use std::sync::atomic::Ordering::{Relaxed, Release};
use std::sync::atomic::AtomicI64;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let mut th_count = 1;
    let mut target = String::new();
    let mut method = String::from("GET");
    let mut args = env::args();
    let mut duration = Duration::from_secs(30);

    while let Some(arg) = args.next() {
        let (flag, val) = match arg.split_once("=") {
            Some((flag, val)) => (flag, Some(val.to_string())),
            None => (arg.as_ref(), None)
        };

        match flag {
            "-t" | "--t" | "-threads" => {
                if let Some(v) = val.or_else(|| args.next()) {
                    th_count = v.parse::<i32>().unwrap();
                }
            }
            "-m" | "--m" | "-method" => {
                if let Some(v) = val.or_else(|| args.next()) {
                    method = v;
                }
            }
            "-d" | "--d" | "-duration" => {
                if let Some(v) = val.or_else(|| args.next()) {
                    duration = Duration::from_secs(v.parse::<u64>().unwrap());
                }
            }
            url if !url.starts_with('-') => {
                target = url.to_string();
            }
            _ => {}
        }
    }
    let (handles, oks, errs) = run_workers(th_count, target, method, duration);
    for handle in handles {
        handle.join().unwrap();
    }
    write_result(oks, errs, duration);

}


fn run_workers(
    th_count: i32,
    target: String,
    method: String,
    dur: Duration,
) -> (Vec<JoinHandle<()>>, Arc<AtomicI64>, Arc<AtomicI64>) {
    let oks = Arc::new(AtomicI64::new(0));
    let errs = Arc::new(AtomicI64::new(0));
    let mut handles = Vec::with_capacity(th_count as usize);

    for _ in 0..th_count {
        let target = target.to_owned();
        let method = method.to_owned();
        let oks = Arc::clone(&oks);
        let errs = Arc::clone(&errs);

        let handle = std::thread::spawn(move || {
            let (ok, err) = send_msg(target, method, dur.clone());
            oks.fetch_add(ok, Release);
            errs.fetch_add(err, Release);
        });

        handles.push(handle);
    }
    (handles, oks, errs)
}
fn write_result(oks: Arc<AtomicI64>, errs: Arc<AtomicI64>, duration: Duration) {
    let ok = oks.load(Relaxed);
    let err = errs.load(Relaxed);
    let all = ok + err;

    let (ok_pct, err_pct) = if all > 0 {
        (
            (ok as f64 / all as f64) * 100.0,
            (err as f64 / all as f64) * 100.0,
        )
    } else {
        (0.0, 0.0)
    };

    let secs = duration.as_secs_f64();
    let rps = if secs > 0.0 { all as f64 / secs } else { 0.0 };

    let oks_cell = format!("{} ({:.2}%)", ok, ok_pct);
    let err_cell = format!("{} ({:.2}%)", err, err_pct);
    let rps_cell = format!("{:.1} req/s", rps);
    let dur_cell = format!("{:?}", duration);
    println!(
        "| {:<10} | {:<20} | {:<20} | {:<15} | {:<12} |\n\

     |------------|----------------------|----------------------|-----------------|--------------|\n\
     | {:<10} | {:<20} | {:<20} | {:<15} | {:<12} |",
        "all", "oks", "errors", "rps", "dur",
        all, oks_cell, err_cell, rps_cell, dur_cell
    );

}