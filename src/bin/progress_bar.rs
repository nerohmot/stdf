use indicatif::ProgressBar;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let bar = ProgressBar::new(100);
    for _ in 0..100 {
        bar.inc(1);
        sleep(Duration::from_millis(50));
    }
    bar.finish();
}