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
                current_timer_mode: TimerMode::Work,
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

            self.remaining_time = self.remaining_time - since_last_pump;

            (self.display_callback)(self.remaining_time);
            self.time_of_last_pump = SystemTime::now();
        }

        fn cycle_mode(&mut self) {
            match self.current_timer_mode {
                TimerMode::Work => {
                    self.work_sessions_since_break = self.work_sessions_since_break + (1 as u8);
                    if self.work_sessions_since_break == self.config.work_sessions_before_long_break
                    {
                        self.current_timer_mode = TimerMode::LongBreak;
                        self.remaining_time = Duration::new(
                            (self.config.long_break_length_in_minutes as u64) * 60,
                            0,
                        );
                        self.work_sessions_since_break = 0;
                        return;
                    } else {
                        self.current_timer_mode = TimerMode::ShortBreak;
                        self.remaining_time = Duration::new(
                            (self.config.short_break_length_in_minutes as u64) * 60,
                            0,
                        );
                        return;
                    }
                }

                TimerMode::ShortBreak => {
                    self.current_timer_mode = TimerMode::Work;
                    self.remaining_time =
                        Duration::new((self.config.work_time_length_in_minutes as u64) * 60, 0);
                    return;
                }

                TimerMode::LongBreak => {
                    //should i quit?
                    self.current_timer_mode = TimerMode::Work;
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
            self.cycle_mode()
        }
        pub fn close_timer(&mut self) -> bool {
            true
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum TimerMode {
        Work,
        ShortBreak,
        LongBreak,
    }

    mod tests {
        use std::{thread::current, time::Duration};

        use crate::trata::TimerMode;

        use super::{Config, TrataTimer};

        fn setup_config() -> Config {
            Config {
                work_time_length_in_minutes: 1,
                short_break_length_in_minutes: 1,
                long_break_length_in_minutes: 1,
                should_have_long_break: true,
                work_sessions_before_long_break: 2,
            }
        }

        fn empty_callback(duration: Duration) {}

        #[test]
        fn timer_startup() {
            let config = setup_config();
            let mut timer = TrataTimer::new(&config, empty_callback);

            timer.start_timer();
            assert!(timer.is_running);

            assert_eq!(timer.current_timer_mode, TimerMode::Work);

            assert_eq!(timer.remaining_time, Duration::new(60, 0));

            timer.close_timer();
        }

        #[test]
        fn timer_cycle() {
            let config = setup_config();
            let mut timer = TrataTimer::new(&config, empty_callback);

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
            let mut timer = TrataTimer::new(&config, empty_callback);
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
            let mut timer = TrataTimer::new(&config, empty_callback);
            timer.start_timer();

            timer.play_pause_timer();
            assert!(!(timer.is_running));
            timer.play_pause_timer();
            assert!(timer.is_running);
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

        #[test]
        #[ignore]
        fn timer_pump() {
            let config = setup_config();
            let mut timer = TrataTimer::new(&config, empty_callback);
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
    }
}

//to do! make tests for methods
