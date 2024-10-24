use std::sync::atomic::{AtomicUsize, Ordering};

use crate::classes::algorithm_params::AlgorithmParams;
use chrono::Local;
use std::path::Path;

pub struct Logger {
    pub counter: AtomicUsize,
    progress_finish_value: f64,
}

impl Logger {
    pub fn new(progress_finish_value: usize) -> Self {
        let counter = AtomicUsize::new(0);

        Self {
            counter,
            progress_finish_value: progress_finish_value as f64,
        }
    }

    fn now(&self) -> String {
        Local::now().format("%H:%M:%S").to_string()
    }

    pub fn log(&self, path: &Path, algo: &AlgorithmParams, status: &str) {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let params = serde_json::to_string(algo).unwrap();
        if status == "END" {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }

        let progress = (self.counter.load(Ordering::SeqCst) as f64 / self.progress_finish_value).floor();

        println!(
            "{} {}% {:>30} {:>95} {:>5}",
            self.now(),
            progress,
            file_name,
            params,
            status
        )
    }
}
