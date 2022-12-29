use std::{thread, time::Duration};

use trata::trata::{Config, TrataTimer};

fn setup_config() -> Config {
    Config {
        work_time_length_in_minutes: 1,
        short_break_length_in_minutes: 1,
        long_break_length_in_minutes: 1,
        should_have_long_break: true,
        work_sessions_before_long_break: 2,
    }
}

fn empty_callback(duration:Duration){

}

#[test]
fn timer_methods(){
    let mut timer = TrataTimer::new(&setup_config(),empty_callback);

    timer.start_timer();
    assert!(timer.)
}
