use merge_mt;
use std::time;

fn main() {
    let mut bytes = merge_mt::arrange_predetermined_random_sequence();
    let tick = time::Instant::now();
    merge_mt::mergesort(&mut bytes[..]);
    let elapsed = tick.elapsed();
    println!(
        "sort time: {}ns => {}us => {}ms => {}s",
        &elapsed.as_nanos(),
        &elapsed.as_micros(),
        &elapsed.as_millis(),
        &elapsed.as_secs()
    );
}
