use std::thread;

use trata::trata::{Config, TrataTimer};

#[test]
fn main() {
    let config: Config = Config {
        work_time_length_in_minutes: 1,
        short_break_length_in_minutes: 1,
        long_break_length_in_minutes: 1,
        should_have_long_break: true,
        work_sessions_before_long_break: 2,
    };

    //atomic something?, make it static?

    let mut timer = TrataTimer::new(&config);

    thread::spawn(|| {
        timer.start_timer();
    });

    loop {
        timer.play_pause_timer();
    }
}
