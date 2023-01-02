extern crate crossterm;

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use std::time::Duration;
use trata::trata::{Config, TimerMode, TrataTimer};

fn main() {
    let config = setup_config();
    let mut timer = TrataTimer::new(&config, display, timer_end_callback);

    timer.start_timer();

    loop {
        //checks for input
        if poll(Duration::from_millis(10)).unwrap() {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            })) = event::read()
            {
                match c {
                    'q' | 'Q' => break,
                    'p' | 'P' => timer.play_pause_timer(),
                    's' | 'S' => timer.end_section_early(),
                    _ => {}
                }
            }
        } else {
        }

        //conducts timer operation (ticking, changing mode, etc)
        timer.pump_timer();
    }
}

//display callback
fn display(duration: Duration, mode: &TimerMode, timer_is_running: bool) {
    //clears the screen
    print!("\x1B[2J\x1B[1;1H");

    //formatting
    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;

    //prints the time remaining in seconds
    println!("{:0>2}:{:0>2}", minutes, seconds);

    if timer_is_running {
        println!("Mode: {}", mode.get_string());
    } else {
        println!("Mode: {} (Paused)", mode.get_string());
    }
    println!("Press Q to quit, S to end current timer early, & P to pause the timer.");
}

fn timer_end_callback(mode: &TimerMode) {}

fn setup_config() -> Config {
    Config {
        work_time_length_in_minutes: 1,
        short_break_length_in_minutes: 1,
        long_break_length_in_minutes: 1,
        should_have_long_break: true,
        should_immediately_transition: false,
        work_sessions_before_long_break: 2,
    }
}
