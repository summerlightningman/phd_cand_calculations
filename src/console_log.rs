use std::sync::atomic::{AtomicUsize, Ordering};

use crate::classes::algorithm_params::AlgorithmParams;
use chrono::Local;
use std::path::Path;

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

    pub fn log_calculation(&self, path: &Path, algo: &AlgorithmParams, status: &str) {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let params = serde_json::to_string(algo).unwrap();
        let current_progress = self.calculate_progress();

        println!(
            "{} {}% {:>30} {:>95} {:>5}",
            self.now(),
            current_progress,
            file_name,
            params,
            status
        )
    }

    pub fn log_file(&self, path: &Path, status: &str) {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if status == "END" {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }

        let current_progress = self.calculate_progress();

        println!(
            "{} {}% {:>30} {:>101}",
            self.now(),
            current_progress,
            file_name,
            status
        )
    }
}
