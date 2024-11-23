use std::sync::atomic::{AtomicUsize, Ordering};

use crate::classes::algorithm_params::AlgorithmParams;
use chrono::Local;

pub struct Logger {
    counter: AtomicUsize,
    progress_target_value: f64,
}

impl Logger {
    pub fn new(progress_start_value: usize, progress_target_value: usize) -> Self {
        let counter = AtomicUsize::new(progress_start_value);
        let progress_target_val = progress_target_value as f64;

        Self {
            counter,
            progress_target_value: progress_target_val,
        }
    }

    fn now(&self) -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    fn calculate_progress(&self) -> f64 {
        (self.counter.load(Ordering::SeqCst) as f64 / self.progress_target_value * 100.0).floor()
    }

    pub fn log_calculation(&self, file_names: &Vec<&str>, algo: &AlgorithmParams, status: &str, calculation_time: Option<i64>) {
        for file_name in file_names {
            let params = serde_json::to_string(algo).unwrap();
            let current_progress = self.calculate_progress();
            let duration = match calculation_time {
               Some(t) => format!("({:.3})", (t as f32) / 1000.0),
               None => String::new()
            };

            println!(
                "{} {}% {:>30} {:^120} {:>5} {}",
                self.now(),
                current_progress,
                file_name,
                params,
                status,
                duration
            )
        }
    }

    pub fn log_file(&self, file_names: &Vec<&str>, status: &str) {
        for file_name in file_names {
            if status == "END" {
                self.counter.fetch_add(1, Ordering::SeqCst);
            }

            let current_progress = self.calculate_progress();

            println!(
                "{} {}% {:>30} {:>126}",
                self.now(),
                current_progress,
                file_name,
                status
            )
        }
    }
}
