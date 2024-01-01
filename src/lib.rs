pub mod trata {
    use std::ops::{Add, AddAssign};

    use chrono::{DateTime, Duration, TimeZone, Utc};
    /*    use std::time::{Duration, SystemTime};

        ///A struct used as the input configuration of a trata timer.
        #[derive(Clone)]
        pub struct Config {
            pub work_length_minutes: u8,
            pub short_break_length_minutes: u8,
            pub long_break_length_minutes: u8,
            pub has_long_break: bool,
            pub timer_mode_will_rollover: bool,
            pub work_sessions_before_long_break: u8,
        }

        ///The distinct modes that a pomodoro timer can be in.
        #[derive(PartialEq, Debug, Clone)]
        pub enum TimerMode {
            Work,
            ShortBreak,
            LongBreak,
        }

        impl TimerMode {
            ///Converts the timer mode into a string for ease of display.
            pub fn get_string(&self) -> String {
                match self {
                    TimerMode::Work => "Work".to_string(),
                    TimerMode::ShortBreak => "Short Break".to_string(),
                    TimerMode::LongBreak => "Long Break".to_string(),
                }
            }
        }

        ///A pomodoro timer struct. Contains methods for running the timer.
        pub struct TrataTimer {
            config: Config,
            current_timer_mode: TimerMode,
            is_running: bool,
            remaining_time: Duration,
            work_sessions_since_break: u8,
            time_of_last_pump: SystemTime,
            display_callback: fn(Duration, &TimerMode, bool),
            end_of_timer_callback: fn(&TimerMode),
        }

        impl TrataTimer {
            ///Takes a Config object and returns a timer that implements it.
            pub fn new(
                configuration: &Config,
                count_down_callback: fn(Duration, &TimerMode, bool),
                rest_timer_callback: fn(&TimerMode),
            ) -> TrataTimer {
                TrataTimer {
                    config: configuration.clone(),
                    current_timer_mode: TimerMode::Work,
                    is_running: false,
                    remaining_time: Duration::new((configuration.work_length_minutes as u64) * 60, 0),
                    time_of_last_pump: SystemTime::now(),
                    display_callback: count_down_callback,
                    end_of_timer_callback: rest_timer_callback,
                    work_sessions_since_break: 0,
                }
            }

            ///Sets the timer to a running state. If you wish the timer to begin display while paused, don't call.
            ///This does not allow the timer to run. In order to run the timer you must loop over the pump function.
            pub fn start_timer(&mut self) {
                self.is_running = true;
                (self.display_callback)(
                    self.remaining_time,
                    &self.current_timer_mode,
                    self.is_running,
                );
            }

            ///Allows the timer to update. Include in a repeating loop.
            pub fn pump_timer(&mut self) {
                if !self.is_running {
                    self.time_of_last_pump = SystemTime::now();
                    return;
                }

                if self.remaining_time <= Duration::ZERO {
                    self.cycle_mode();
                    self.time_of_last_pump = SystemTime::now();
                    return;
                }

                let since_last_pump: Duration =
                    match SystemTime::now().duration_since(self.time_of_last_pump) {
                        Ok(value) => value,
                        Err(_) => panic!("Error calculating time since last pump."),
                    };
                if since_last_pump < Duration::new(0, 1000000000) {
                    return;
                }

                self.remaining_time = match self.remaining_time.checked_sub(since_last_pump) {
                    Some(value) => value,
                    None => {
                        self.cycle_mode();
                        (self.display_callback)(
                            self.remaining_time,
                            &self.current_timer_mode,
                            self.is_running,
                        );
                        self.remaining_time
                    }
                };

                if self.is_running {
                    (self.display_callback)(
                        self.remaining_time,
                        &self.current_timer_mode,
                        self.is_running,
                    );
                }
                self.time_of_last_pump = SystemTime::now();
            }

            fn cycle_mode(&mut self) {
                match self.current_timer_mode {
                    TimerMode::Work => {
                        self.work_sessions_since_break += 1_u8;
                        if self.work_sessions_since_break == self.config.work_sessions_before_long_break
                        {
                            self.set_up_new_mode(TimerMode::LongBreak);
                            self.work_sessions_since_break = 0;
                        } else {
                            self.set_up_new_mode(TimerMode::ShortBreak);
                        }
                    }

                    TimerMode::ShortBreak => {
                        self.set_up_new_mode(TimerMode::Work);
                    }

                    TimerMode::LongBreak => {
                        self.set_up_new_mode(TimerMode::Work);
                        self.work_sessions_since_break = 0;
                        //this should quit right?
                    }
                }

                if !self.config.timer_mode_will_rollover {
                    self.is_running = false;
                }

                (self.end_of_timer_callback)(&self.current_timer_mode);

                (self.display_callback)(
                    self.remaining_time,
                    &self.current_timer_mode,
                    self.is_running,
                );
            }

            fn set_up_new_mode(&mut self, new_mode: TimerMode) {
                self.current_timer_mode = new_mode.clone();
                match new_mode {
                    TimerMode::Work => {
                        self.remaining_time =
                            Duration::new((self.config.work_length_minutes as u64) * 60, 0)
                    }
                    TimerMode::ShortBreak => {
                        self.remaining_time =
                            Duration::new((self.config.short_break_length_minutes as u64) * 60, 0)
                    }
                    TimerMode::LongBreak => {
                        self.remaining_time =
                            Duration::new((self.config.long_break_length_minutes as u64) * 60, 0)
                    }
                }
            }

            ///Toggles the timer between running, and paused.
            pub fn play_pause_timer(&mut self) {
                if self.is_running {
                    self.is_running = false;
                    (self.display_callback)(
                        self.remaining_time,
                        &self.current_timer_mode,
                        self.is_running,
                    );
                } else {
                    self.is_running = true;
                    (self.display_callback)(
                        self.remaining_time,
                        &self.current_timer_mode,
                        self.is_running,
                    );
                }
            }

            ///Ends the
            pub fn end_section_early(&mut self) {
                self.cycle_mode();
                if !self.config.timer_mode_will_rollover {
                    self.is_running = false;
                }
            }
        }
    */
    pub struct Timer {
        start_time: DateTime<Utc>,
        expected_end_time: DateTime<Utc>,
        time_of_pause: Option<DateTime<Utc>>,
    }

    impl Timer {
        pub fn start_from_duration(length: Duration) -> Timer {
            let now = Utc::now();
            return Timer {
                start_time: now,
                expected_end_time: now + length,
                time_of_pause: None,
            };
        }

        pub fn get_timer_state(&self) -> TimerState {
            let now = Utc::now();

            match self.time_of_pause {
                Some(pause_time) => {
                    return TimerState::Paused {
                        remaining_duration: self.expected_end_time - pause_time,
                    }
                }
                None => (),
            }

            if now > self.expected_end_time {
                return TimerState::Complete;
            }

            return TimerState::Running {
                remaining_duration: self.expected_end_time - now,
            };
        }

        pub fn pause(&mut self) {
            match self.time_of_pause {
                Some(pause_time) => {
                    self.expected_end_time += Utc::now() - pause_time;
                    self.time_of_pause = None
                }
                None => self.time_of_pause = Some(Utc::now()),
            }
        }

        pub fn end_early(&mut self) {
            self.expected_end_time = Utc::now();
        }
    }

    #[derive(PartialEq, Eq, Debug)]
    pub enum TimerState {
        Running { remaining_duration: Duration },
        Paused { remaining_duration: Duration },
        Complete,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::mem;

        #[test]
        fn get_timer_state_start_timer_assert_running() {
            let timer = Timer::start_from_duration(Duration::seconds(2));
            let result = timer.get_timer_state();
            assert_eq!(
                mem::discriminant(&TimerState::Running {
                    remaining_duration: Duration::seconds(2)
                }),
                mem::discriminant(&result),
                "The timer state did not default to running"
            );
        }

        #[test]
        fn get_timer_state_pause_timer_assert_paused() {
            let mut timer = Timer::start_from_duration(Duration::seconds(2));
            timer.pause();
            let result = timer.get_timer_state();
            assert_eq!(
                mem::discriminant(&TimerState::Paused {
                    remaining_duration: Duration::seconds(2)
                }),
                mem::discriminant(&result),
                "The timer was paused but did not return the paused state"
            );
        }

        #[test]
        fn get_timer_state_pause_timer_assert_not_running() {
            let mut timer = Timer::start_from_duration(Duration::seconds(2));
            timer.pause();
            let result = timer.get_timer_state();
            assert_ne!(
                mem::discriminant(&TimerState::Running {
                    remaining_duration: Duration::seconds(2)
                }),
                mem::discriminant(&result),
                "The timer was running after being paused"
            );
        }
    }
    /*
            fn setup_config() -> Config {
                Config {
                    work_length_minutes: 1,
                    short_break_length_minutes: 1,
                    long_break_length_minutes: 1,
                    has_long_break: true,
                    timer_mode_will_rollover: true,
                    work_sessions_before_long_break: 2,
                }
            }

            fn run_timer(timer: &mut TrataTimer) {
                let held_timer_mode = timer.current_timer_mode.clone();

                loop {
                    timer.pump_timer();
                    if timer.current_timer_mode != held_timer_mode {
                        break;
                    }
                    if timer.remaining_time < Duration::new(0, 0) {
                        panic!("Timer ran out but mode didn't change");
                    }
                }
            }

            fn empty_timer_callback(duration: Duration, mode: &TimerMode, timer_is_running: bool) {}
            fn empty_callback(work_mode: &TimerMode) {}

            #[test]
            fn timer_startup() {
                let config = setup_config();
                let mut timer = TrataTimer::new(&config, empty_timer_callback, empty_callback);

                timer.start_timer();
                assert!(timer.is_running);

                assert_eq!(timer.current_timer_mode, TimerMode::Work);

                assert_eq!(timer.remaining_time, Duration::new(60, 0));
            }

            #[test]
            fn timer_cycle() {
                let config = setup_config();
                let mut timer = TrataTimer::new(&config, empty_timer_callback, empty_callback);

                assert_eq!(timer.current_timer_mode, TimerMode::Work);

                timer.cycle_mode();
                assert_eq!(timer.current_timer_mode, TimerMode::ShortBreak);

                timer.cycle_mode();
                assert_eq!(timer.current_timer_mode, TimerMode::Work);

                timer.cycle_mode();
                assert_eq!(
                    timer.current_timer_mode,
                    TimerMode::LongBreak,
                    "Sessions since break: {}, Config: {}",
                    timer.work_sessions_since_break,
                    timer.config.work_sessions_before_long_break
                );
            }

            #[test]
            fn timer_end_early() {
                let config = setup_config();
                let mut timer = TrataTimer::new(&config, empty_timer_callback, empty_callback);
                timer.start_timer();

                timer.end_section_early();
                assert_eq!(timer.current_timer_mode, TimerMode::ShortBreak);

                timer.end_section_early();
                assert_eq!(timer.current_timer_mode, TimerMode::Work);

                timer.end_section_early();
                assert_eq!(timer.current_timer_mode, TimerMode::LongBreak);
            }

            #[test]
            fn timer_play_pause() {
                let config = setup_config();
                let mut timer = TrataTimer::new(&config, empty_timer_callback, empty_callback);
                timer.start_timer();

                timer.play_pause_timer();
                assert!(!(timer.is_running));
                timer.play_pause_timer();
                assert!(timer.is_running);
            }

            #[test]
            #[ignore]
            fn timer_pump() {
                let config = setup_config();
                let mut timer = TrataTimer::new(&config, empty_timer_callback, empty_callback);
                timer.start_timer();

                //Timer started, should be in work mode
                assert_eq!(
                    timer.current_timer_mode,
                    TimerMode::Work,
                    "Timer just started, should be in work mode."
                );
                assert_eq!(
                    timer.remaining_time,
                    Duration::new(60, 0),
                    "Default config defines each timer mode as one minute."
                );

                timer.remaining_time = Duration::new(1, 0);
                run_timer(&mut timer);

                //Finished first work session, timer should enter a short break
                assert_eq!(
                    timer.current_timer_mode,
                    TimerMode::ShortBreak,
                    "Timer finished a work session, should be in short break mode."
                );
                assert_eq!(
                    timer.remaining_time,
                    Duration::new(60, 0),
                    "Default config defines each timer mode as one minute."
                );

                timer.remaining_time = Duration::new(1, 0);
                run_timer(&mut timer);

                //finished first short break, timer should enter work mode
                assert_eq!(
                    timer.current_timer_mode,
                    TimerMode::Work,
                    "Finished the first short break, should be in work mode."
                );
                assert_eq!(
                    timer.remaining_time,
                    Duration::new(60, 0),
                    "Default config defines each timer mode as one minute."
                );

                timer.remaining_time = Duration::new(1, 0);
                run_timer(&mut timer);

                //finished second work session, should enter long break
                assert_eq!(
                    timer.current_timer_mode,
                    TimerMode::LongBreak,
                    "Finished second work mode, should be in Long Break mode."
                );
                assert_eq!(
                    timer.remaining_time,
                    Duration::new(60, 0),
                    "Default config defines each timer mode as one minute."
                );
            }

            #[test]
            #[ignore]
            fn timer_rollover() {
                let conf = Config {
                    work_length_minutes: 1,
                    short_break_length_minutes: 1,
                    long_break_length_minutes: 1,
                    has_long_break: true,
                    timer_mode_will_rollover: false,
                    work_sessions_before_long_break: 2,
                };

                let mut timer = TrataTimer::new(&conf, empty_timer_callback, empty_callback);
                timer.start_timer();
                timer.remaining_time = Duration::new(1, 0);
                run_timer(&mut timer);
                assert!(!timer.is_running)
            }
        }

    */
}
