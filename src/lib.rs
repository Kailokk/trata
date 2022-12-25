use chrono;
use std::error::Error;

pub mod trata {

    pub struct Config {work_time_length_in_minutes : u8, short_break_length_in_minutes : u8, long_break_length_in_minutes : u8, should_have_long_break : bool, work_sessions_before_long_break : u8}}

    impl Config {
        fn new() { }
    }

    pub struct TrataTimer {}

    impl TrataTimer {
        //should take an anoymous function as a timer callback
        fn new(configuration: Config) -> TrataTimer {}

        fn start_timer(&self) {}
        fn pause_timer(&self) {}
        fn end_section_early(&self) {}
        fn close_timer(&self) {}
    }
}
