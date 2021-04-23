use core::time;
use std::{
    fmt,
    io::{self, Write},
    sync::mpsc::{self, Sender},
};

//add a progress circle maybe make progres bar struct with fn print/ Display
// 3 collumns for procent
// 1 for - before procent
// 2 for brackets
#[allow(dead_code)]
pub fn print_progress_bar(width: i32, progress: f64) {
    let inner_width = width - 6;
    let step = 100.0 / (inner_width as f64);
    let num_filed_fields = (progress * 100.0 / step) as i32;
    print!("\r");
    print!("[");
    for _ in 0..num_filed_fields {
        print!("*");
    }
    for _ in 0..(inner_width - num_filed_fields) {
        print!(".")
    }
    print!("]");
    print!("-{:>3}%", (progress * 100.0) as usize);
    io::stdout().flush().unwrap();
}

pub struct NotStartedError;
impl fmt::Display for NotStartedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "async progress bar not started")
    }
}
pub struct AsyncProgressBar {
    channel: Result<Sender<()>, NotStartedError>,
    chars: Vec<String>,
    interval: time::Duration,
    label: String,
}

impl AsyncProgressBar {
    pub fn start(chars: Vec<String>, interval: time::Duration, label: String) -> AsyncProgressBar {
        let mut pb = AsyncProgressBar {
            channel: Result::Err(NotStartedError),
            chars: chars,
            interval: interval,
            label: label,
        };
        pb.start_circle();
        pb
    }

    fn start_circle(&mut self) {
        let cycle = self.chars.clone();
        let label = self.label.clone();
        let interval = self.interval.clone();
        let (tx, rx) = mpsc::channel::<()>();
        self.channel = Result::Ok(tx);
        std::thread::spawn(move || {
            let c = cycle.iter().cycle();
            for ch in c {
                print!("\r{} - {}", ch, label);
                std::io::stdout().flush().unwrap();
                std::thread::sleep(interval);
                match rx.try_recv() {
                    Ok(_) => {
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => {}
                    Err(mpsc::TryRecvError::Disconnected) => {
                        std::thread::sleep(interval);
                    }
                }
            }
        });
    }

    pub fn stop(self) -> Result<(), NotStartedError> {
        match self.channel {
            Ok(_) => {
                std::thread::sleep(self.interval);
                print!("\r{: <1$}\r", "", self.label.len() + 4); // + chars width + 3 for ` - `
                Ok(())
            }
            Err(NotStartedError) => return Err(NotStartedError),
        }
    }
}
