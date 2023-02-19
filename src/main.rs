use merge_mt;
use std::env;
use std::process;
use std::time;

pub struct Config {
    count: u32,
    threads: u32,
}

impl Config {
    pub fn build(args: Vec<String>) -> Self {
        let args_len = args.len();
        if args_len < 2 || args_len > 3 {
            eprintln!("received an incorrect number of arguments");
            eprintln!("Usage: {} <count> [threads]", args[0]);
            process::exit(1);
        }
        let mut args_iter = args.iter();
        args_iter.next();
        let count = args_iter
            .next()
            .unwrap()
            .parse::<u32>()
            .expect("argument for `count` must be an integer");
        let threads = match args_iter.next() {
            Some(str) => str
                .parse::<u32>()
                .expect("argument for `threads` was not an integer"),
            None => 1,
        };

        Self { count, threads }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args);
    run(config);
}
pub fn run(config: Config) {
    let mut bytes = merge_mt::arrange_deterministic_random_sequence(config.count as usize);
    let tick = time::Instant::now();
    let _sorted_bytes = merge_mt::mergesort_mt(&mut bytes, config.threads as usize);
    let elapsed = tick.elapsed();

    println!(
        "sort time: {}ns => {}us => {}ms => {}s",
        &elapsed.as_nanos(),
        &elapsed.as_micros(),
        &elapsed.as_millis(),
        &elapsed.as_secs()
    );
}
