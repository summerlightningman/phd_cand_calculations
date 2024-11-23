mod classes;
mod console_log;
mod file_opener;
mod types;

use chrono::prelude::*;
use rayon::prelude::*;
use regex::Regex;

use csv::Writer;
use ndarray::Array2;
use ndarray_npy::ReadNpyExt;
use types::{FileRow, SenderInfo};

use std::cmp::Ordering;
use std::env;
use std::fs::DirEntry;
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
use phd_cand_algorithms::types::{Purpose, Task};
use std::io::prelude::*;

const MATRICES_DIR: &'static str = "matrices";
const MATRICES_COUNT_TARGET_DEFAULT: usize = 100;

fn process_matrix(
    logger: Arc<Logger>,
    distance_path: &Path,
    time_path: &Path,
    importance_path: &Path,
    csv_sender: Sender<SenderInfo>,
) {
    let mut tasks: Vec<Task> = vec![];
    let mut file_names: Vec<&str> = vec![];

    for (name, path, purpose) in vec![
        ("distance", distance_path, Purpose::Min),
        ("time", time_path, Purpose::Min),
        ("importance", importance_path, Purpose::Max),
    ] {
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
        }
        .round();

        let matrix_vec: Vec<Vec<f64>> = matrix.outer_iter().map(|row| row.to_vec()).collect();

        tasks.push(Task {
            name: name.to_string(),
            matrix: matrix_vec,
            purpose,
        });
        file_names.push(path.file_name().unwrap().to_str().unwrap())
    }

    logger.log_file(&file_names, "START");

    for params in ALGORITHMS {
        logger.log_calculation(&file_names, &params, "START", None);

        let dataset_row = match run_algo(params.clone(), tasks.clone()) {
            Some(s) => s,
            None => {
                logger.log_calculation(&file_names, &params, "ERROR", None);
                continue;
            }
        };

        logger.log_calculation(
            &file_names,
            &params,
            "END",
            Some(dataset_row.calculation_time),
        );

        let _ = csv_sender.send(SenderInfo::DatasetRow(dataset_row));
    }

    logger.log_file(&file_names, "END");

    for file_name in file_names {
        let _ = csv_sender.send(SenderInfo::FileRow(FileRow(file_name.to_string())));
    }
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
                    continue;
                }
                writer
                    .serialize(row)
                    .expect("Не удалось записать результат");
            }
        }
    }
}

fn are_sizes_equal(
    pattern: &Regex,
    distance_path: &DirEntry,
    time_path: &DirEntry,
    importance_path: &DirEntry,
) -> bool {
    let (dist_filename, time_filename, impo_filename) = (
        distance_path.file_name(),
        time_path.file_name(),
        importance_path.file_name(),
    );

    let dist_caps = pattern.captures(dist_filename.to_str().unwrap()).unwrap();
    let time_caps = pattern.captures(time_filename.to_str().unwrap()).unwrap();
    let impo_caps = pattern.captures(impo_filename.to_str().unwrap()).unwrap();

    let dist_size = dist_caps.name("size").unwrap().as_str().parse::<usize>();
    let time_size = time_caps.name("size").unwrap().as_str().parse::<usize>();
    let impo_size = impo_caps.name("size").unwrap().as_str().parse::<usize>();

    return dist_size == time_size && time_size == impo_size;
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
    let mut matrices_paths: Vec<_> = fs::read_dir(matrices_path)
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

    let pattern = Regex::new(r".+_(?<size>\d+)\.npy").unwrap();
    matrices_paths.sort_by(|a, b| {
        let (a_filename, b_filename) = match (a, b) {
            (Ok(a_file), Ok(b_file)) => (a_file.file_name(), b_file.file_name()),
            _ => return Ordering::Equal,
        };

        let a_caps = pattern.captures(a_filename.to_str().unwrap()).unwrap();
        let b_caps = pattern.captures(b_filename.to_str().unwrap()).unwrap();

        let a_size = a_caps
            .name("size")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let b_size = b_caps
            .name("size")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        a_size.cmp(&b_size)
    });

    let (result_sender, result_receiver) = mpsc::channel();
    let writer_thread = thread::spawn(move || writer_handle(result_receiver, file_manager));

    let calculation_dt_start = Local::now();

    matrices_paths.par_iter().chunks(3).for_each(|chunk| {
        if let [Ok(distance), Ok(time), Ok(importance)] = chunk[..] {
            if are_sizes_equal(&pattern, distance, time, importance) {
                process_matrix(
                    logger.clone(),
                    distance.path().as_path(),
                    time.path().as_path(),
                    importance.path().as_path(),
                    result_sender.clone(),
                )
            }
        }
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
