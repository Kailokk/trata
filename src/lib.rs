pub mod trata {
    use std::error::Error;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    #[derive(Clone)]
    pub struct Config {
        pub work_time_length_in_minutes: u8,
        pub short_break_length_in_minutes: u8,
        pub long_break_length_in_minutes: u8,
        pub should_have_long_break: bool,
        pub work_sessions_before_long_break: u8,
    }

    pub struct TrataTimer {
        config: Config,
        current_timer_mode: TimerMode,
        is_running: bool,
        remaining_time: Duration,
        time_of_last_pump: SystemTime,
        display_callback: fn(Duration),
        work_sessions_since_break: u8,
    }

    impl TrataTimer {
        //should take an anoymous function as a timer callback
        pub fn new(configuration: &Config, callback: fn(Duration)) -> TrataTimer {
            TrataTimer {
                config: configuration.clone(),
                current_timer_mode: TimerMode::work,
                is_running: false,
                remaining_time: Duration::new(
                    (configuration.work_time_length_in_minutes as u64) * 60,
                    0,
                ),
                time_of_last_pump: SystemTime::now(),
                display_callback: callback,
                work_sessions_since_break: 0,
            }
        }

        pub fn start_timer(&mut self) {
            self.is_running = true;
        }

        pub fn pump_timer(&mut self) {
            if !self.is_running {
                return;
            }

            if self.remaining_time.is_zero() {
                self.cycle_mode();
                return;
            }

            let since_last_pump: Duration = SystemTime::now()
                .duration_since(self.time_of_last_pump)
                .unwrap();

            self.remaining_time = self.remaining_time.checked_sub(since_last_pump).unwrap();
            (self.display_callback)(self.remaining_time);
            self.time_of_last_pump = SystemTime::now();
        }

        fn cycle_mode(&mut self) {
            match self.current_timer_mode {
                TimerMode::work => {
                    self.work_sessions_since_break = self.work_sessions_since_break + 1;
                    if self.work_sessions_since_break >= self.config.work_sessions_before_long_break
                    {
                        self.current_timer_mode = TimerMode::long_break;
                        self.remaining_time = Duration::new(
                            (self.config.long_break_length_in_minutes as u64) * 60,
                            0,
                        );
                        self.work_sessions_since_break = 0;
                        return;
                    } else {
                        self.current_timer_mode = TimerMode::short_break;
                        self.remaining_time = Duration::new(
                            (self.config.short_break_length_in_minutes as u64) * 60,
                            0,
                        );
                        return;
                    }
                }
                TimerMode::short_break => {
                    self.current_timer_mode = TimerMode::work;
                    self.remaining_time =
                        Duration::new((self.config.work_time_length_in_minutes as u64) * 60, 0);
                    self.work_sessions_since_break = 0;
                    return;
                }
                TimerMode::long_break => {
                    self.current_timer_mode = TimerMode::work;
                    self.remaining_time =
                        Duration::new((self.config.work_time_length_in_minutes as u64) * 60, 0);
                    self.work_sessions_since_break = 0;
                    return;
                }
            }
        }

        pub fn play_pause_timer(&mut self) {
            if self.is_running {
                self.is_running = false;
            } else {
                self.is_running = true;
            }
        }

        pub fn end_section_early(&mut self) {
            //change to next cycle
        }
        pub fn close_timer(&mut self) -> bool {
            true
        }
    }

    pub enum TimerMode {
        work,
        short_break,
        long_break,
    }
}

//to do! make tests for methods
