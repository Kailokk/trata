use chrono;
use std::error::Error;

pub mod trata {

    #[derive(Clone)]
    pub struct Config {
        work_time_length_in_minutes: u8,
        short_break_length_in_minutes: u8,
        long_break_length_in_minutes: u8,
        should_have_long_break: bool,
        work_sessions_before_long_break: u8,
    }

    pub struct TrataTimer {
        config: Config,
        current_timer_mode: TimerMode,
        is_running: bool,
        remaining_time_in_seconds: usize,
    }

    impl TrataTimer {
        //should take an anoymous function as a timer callback
        pub fn new(configuration: &Config) -> TrataTimer {
            TrataTimer {
                config: configuration.clone(),
                current_timer_mode: TimerMode::work {
                    length_in_minutes: configuration.work_time_length_in_minutes,
                },
                is_running: false,
                remaining_time_in_seconds: (configuration.work_time_length_in_minutes as usize)
                    * 60,
            }
        }

        pub fn start_timer(&self) {}
        pub fn play_pause_timer(&mut self) {
            if self.is_running {
                self.pause_timer();
                self.is_running = false;
            } else {
                self.play_timer();
                self.is_running = true;
            }
        }

        fn play_timer(&mut self) {}
        fn pause_timer(&mut self) {}
        pub fn end_section_early(&self) {}
        pub fn close_timer(&self) {}
    }

    pub enum TimerMode {
        work { length_in_minutes: u8 },
        short_break { length_in_minutes: u8 },
        long_break { length_in_minutes: u8 },
    }
}
