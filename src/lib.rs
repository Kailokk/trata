pub mod trata {
    use std::time::{Duration, SystemTime};

    #[derive(Clone)]
    pub struct Config {
        pub work_time_length_in_minutes: u8,
        pub short_break_length_in_minutes: u8,
        pub long_break_length_in_minutes: u8,
        pub should_have_long_break: bool,
        pub should_immediately_transition: bool,
        pub work_sessions_before_long_break: u8,
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum TimerMode {
        Work,
        ShortBreak,
        LongBreak,
    }

    impl TimerMode {
        pub fn get_string(&self) -> String {
            match self {
                TimerMode::Work => "Work".to_string(),
                TimerMode::ShortBreak => "Short Break".to_string(),
                TimerMode::LongBreak => "Long Break".to_string(),
            }
        }
    }

    pub struct TrataTimer {
        config: Config,
        current_timer_mode: TimerMode,
        is_running: bool,
        remaining_time: Duration,
        time_of_last_pump: SystemTime,
        display_callback: fn(Duration, &TimerMode, bool),
        end_of_timer_callback: fn(&TimerMode),
        work_sessions_since_break: u8,
    }

    impl TrataTimer {
        pub fn new(
            configuration: &Config,
            count_down_callback: fn(Duration, &TimerMode, bool),
            rest_timer_callback: fn(&TimerMode),
        ) -> TrataTimer {
            TrataTimer {
                config: configuration.clone(),
                current_timer_mode: TimerMode::Work,
                is_running: false,
                remaining_time: Duration::new(
                    (configuration.work_time_length_in_minutes as u64) * 60,
                    0,
                ),
                time_of_last_pump: SystemTime::now(),
                display_callback: count_down_callback,
                end_of_timer_callback: rest_timer_callback,
                work_sessions_since_break: 0,
            }
        }

        pub fn start_timer(&mut self) {
            self.is_running = true;
        }

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

            let since_last_pump: Duration = SystemTime::now()
                .duration_since(self.time_of_last_pump)
                .unwrap();

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

            if !self.config.should_immediately_transition {
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
                        Duration::new((self.config.work_time_length_in_minutes as u64) * 60, 0)
                }
                TimerMode::ShortBreak => {
                    self.remaining_time =
                        Duration::new((self.config.long_break_length_in_minutes as u64) * 60, 0)
                }
                TimerMode::LongBreak => {
                    self.remaining_time =
                        Duration::new((self.config.short_break_length_in_minutes as u64) * 60, 0)
                }
            }
        }

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

        pub fn end_section_early(&mut self) {
            self.cycle_mode();
            if !self.config.should_immediately_transition {
                self.is_running = false;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn setup_config() -> Config {
            Config {
                work_time_length_in_minutes: 1,
                short_break_length_in_minutes: 1,
                long_break_length_in_minutes: 1,
                should_have_long_break: true,
                should_immediately_transition: true,
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
                work_time_length_in_minutes: 1,
                short_break_length_in_minutes: 1,
                long_break_length_in_minutes: 1,
                should_have_long_break: true,
                should_immediately_transition: false,
                work_sessions_before_long_break: 2,
            };

            let mut timer = TrataTimer::new(&conf, empty_timer_callback, empty_callback);
            timer.start_timer();
            timer.remaining_time = Duration::new(1, 0);
            run_timer(&mut timer);
            assert!(!timer.is_running)
        }
    }
}
