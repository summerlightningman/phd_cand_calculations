mod classes;
mod console_log;
mod file_opener;
mod types;

use chrono::prelude::*;
use rayon::prelude::*;

use csv::Writer;
use ndarray::Array2;
use ndarray_npy::ReadNpyExt;
use types::{FileRow, SenderInfo};

use std::env;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use std::{
    env::current_dir,
    fs::{self, File},
    path::Path,
    thread,
};

use classes::algorithms::ALGORITHMS;
use classes::run_algo::run_algo;

use console_log::Logger;
use file_opener::FileManager;
use std::io::prelude::*;

const MATRICES_DIR: &'static str = "matrices";
const MATRICES_COUNT_TARGET_DEFAULT: usize = 100;

fn process_matrix(logger: Arc<Logger>, path: &Path, csv_sender: Sender<SenderInfo>) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_) => {
            println!("Не удалось открыть файл {:?}", path);
            return;
        }
    };

    let matrix = match Array2::<f64>::read_npy(file) {
        Ok(mt) => mt,
        Err(_) => return,
    };

    logger.log_file(path, "START");

    let matrix_vec: Vec<Vec<f64>> = matrix.outer_iter().map(|row| row.to_vec()).collect();

    let file_name: String = path.file_name().unwrap().to_str().unwrap().to_string();
    for params in ALGORITHMS {
        logger.log_calculation(path, &params, "START");

        let dataset_row = match run_algo(params.clone(), matrix_vec.clone()) {
            Ok(s) => s,
            Err(_) => {
                logger.log_calculation(path, &params, "ERROR");
                continue;
            }
        };

        logger.log_calculation(path, &params, "END");

        let _ = csv_sender.send(SenderInfo::DatasetRow(dataset_row));
    }

    logger.log_file(path, "END");

    let _ = csv_sender.send(SenderInfo::FileRow(FileRow(file_name.clone())));
}

fn writer_handle(receiver: Receiver<SenderInfo>, file_manager: FileManager) {
    let FileManager {
        dataset_file,
        mut log_file,
        ..
    } = file_manager;
    let mut writer = Writer::from_writer(dataset_file);

    for result in receiver {
        match result {
            SenderInfo::FileRow(FileRow(file_path)) => {
                log_file
                    .write(format!("{}\n", file_path).as_bytes())
                    .expect(format!("Unable to write {}", file_path).as_str());
            }
            SenderInfo::DatasetRow(row) => {
                if row.iterations.is_empty() {
                    continue
                }
                writer
                    .serialize(row)
                    .expect("Не удалось записать результат");
            }
        }
    }
}

fn main() {
    let matrices_count: usize = match env::var("MATRICES_COUNT") {
        Ok(val) => val.parse().unwrap_or(MATRICES_COUNT_TARGET_DEFAULT),
        Err(_) => MATRICES_COUNT_TARGET_DEFAULT,
    };

    let file_manager = FileManager::new(matrices_count);
    let logger = Arc::new(Logger::new(file_manager.log_entries.len(), matrices_count));
    let log_entries = file_manager.log_entries.clone();

    let curr_dir = current_dir().unwrap();
    let matrices_path = curr_dir.join(MATRICES_DIR);
    let matrices_paths: Vec<_> = fs::read_dir(matrices_path)
        .unwrap()
        .filter(|dir_entry| {
            if log_entries.is_empty() {
                true
            } else if let Ok(entry) = dir_entry {
                log_entries.contains(entry.file_name().to_str().unwrap())
            } else {
                false
            }
        })
        .take(matrices_count - log_entries.len())
        .collect();

    let (result_sender, result_receiver) = mpsc::channel();
    let writer_thread = thread::spawn(move || writer_handle(result_receiver, file_manager));

    let calculation_dt_start = Local::now();

    matrices_paths.par_iter().for_each(|matrix| {
        let path = match matrix {
            Ok(dir_entry) => dir_entry.path(),
            Err(_) => return,
        };

        process_matrix(logger.clone(), path.as_path(), result_sender.clone());
    });

    drop(result_sender);

    writer_thread
        .join()
        .expect("writer handle завершился с ошибкой");

    let calculation_dt_end = Local::now();
    let duration = calculation_dt_end.signed_duration_since(calculation_dt_start);
    println!(
        "Calculation finished in {}d {}h {}m {}s",
        duration.num_days(),
        duration.num_hours() % 24,
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    )
}
