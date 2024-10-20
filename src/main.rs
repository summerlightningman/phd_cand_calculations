mod classes;
mod dataset;
mod types;

use chrono::prelude::*;
use rayon::prelude::*;

use classes::algorithm_params::AlgorithmParams;
use csv::Writer;
use ndarray::Array2;
use ndarray_npy::ReadNpyExt;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::{
    env::current_dir,
    fs::{self, File},
    path::Path,
    thread,
};

use classes::algorithms::ALGORITHMS;
use classes::run_algo::run_algo;

use dataset::DatasetRow;

const MATRICES_COUNT: usize = 100;
const DATASET_DIR: &'static str = "matrices";
static LOG_CALL_COUNT: AtomicUsize = AtomicUsize::new(0);
static PROGRESS_FINISH_VALUE: f64 = (MATRICES_COUNT * ALGORITHMS.len()) as f64;

fn log(path: &Path, algo: &AlgorithmParams, status: &str) {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let params = serde_json::to_string(algo).unwrap();
    if status.eq("END") {
        LOG_CALL_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    let progress =
        (LOG_CALL_COUNT.load(Ordering::SeqCst) as f64 / PROGRESS_FINISH_VALUE * 100.0f64).floor();

    println!(
        "{} {}% {:>30} {:>95} {:>5}",
        now(),
        progress,
        file_name,
        params,
        status
    )
}

fn process_matrix(path: &Path, csv_sender: Sender<DatasetRow>) {
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

    let matrix_vec: Vec<Vec<f64>> = matrix.outer_iter().map(|row| row.to_vec()).collect();

    for params in ALGORITHMS {
        log(path, &params, "START");
        let solutions = run_algo(params.clone(), matrix_vec.clone());
        let solutions_unwrapped = match solutions {
            Ok(s) if s.len() < 1 => {
                log(path, &params, "ERROR");
                continue;
            }
            Ok(s) => s,
            Err(_) => {
                log(path, &params, "ERROR");
                continue;
            }
        };

        log(path, &params, "END");

        let row = DatasetRow::new(
            params,
            matrix_vec.clone(),
            solutions_unwrapped[0].solution.fitness,
        );
        let _ = csv_sender.send(row);
    }
}

fn now() -> String {
    Utc::now().format("%H:%M:%S").to_string()
}

fn generate_csv_filename() -> String {
    Utc::now().format("dataset_%Y-%m-%d_%H:00.csv").to_string()
}

fn writer_handle(receiver: Receiver<DatasetRow>, output_file: String) {
    let file = File::create(output_file).expect("Не удалось создать файл");
    let mut writer = Writer::from_writer(file);

    // Получаем результаты и записываем их
    for result in receiver {
        writer
            .serialize(result)
            .expect("Не удалось записать результат");
    }
}

fn main() {
    let current_dir = current_dir().unwrap();
    let dataset_path = current_dir.join(DATASET_DIR);
    let matrices_paths: Vec<_> = fs::read_dir(dataset_path)
        .unwrap()
        .take(MATRICES_COUNT)
        .collect();

    let (result_sender, result_receiver) = mpsc::channel();

    let writer_thread =
        thread::spawn(move || writer_handle(result_receiver, generate_csv_filename()));

    let calculation_dt_start = Utc::now();
    matrices_paths.par_iter().for_each(|matrix| {
        let path = match matrix {
            Ok(dir_entry) => dir_entry.path(),
            Err(_) => return,
        };

        let csv_sender = result_sender.clone();

        process_matrix(path.as_path(), csv_sender);
    });

    drop(result_sender);

    writer_thread
        .join()
        .expect("writer handle завершился с ошибкой");

    let calculation_dt_end = Utc::now();
    let duration = calculation_dt_end.signed_duration_since(calculation_dt_start);
    println!(
        "Calculation finished in {}d {}h {}m {}s",
        duration.num_days(),
        duration.num_hours() % 24,
        duration.num_minutes() % 60,
        duration.num_seconds() % 60
    )
}
