use chrono::Local;
use regex::Regex;
use std::{
    collections::HashSet,
    env::current_dir,
    fs::{self, DirEntry, File, OpenOptions},
    io::{BufRead, BufReader},
    time::UNIX_EPOCH,
};

pub struct FileManager {
    pub log_file: File,
    pub dataset_file: File,
    pub log_entries: HashSet<String>,
}

impl FileManager {
    pub fn new(matrices_count: usize) -> Self {
        let now = Self::now();
        let newest_log_file = Self::get_log_file_path();

                if let Some(log_path) = newest_log_file {
            let old_log_file = File::open(&log_path);

            if let Ok(old_log_file) = old_log_file {
                let old_file_count_finished = BufReader::new(&old_log_file).lines().count();
                let pattern = Regex::new(r"log_(?<count>\d+)_(?<dt>.*).csv").unwrap();
                let caps = pattern.captures(&log_path);
                if let Some(caps) = caps {
                    let old_file_count_target = caps
                        .name("count")
                        .unwrap()
                        .as_str()
                        .parse::<usize>()
                        .unwrap();
                    if old_file_count_finished < old_file_count_target
                        && old_file_count_target == matrices_count
                    {
                        let log_file = OpenOptions::new().append(true).read(true).open(&log_path);
                        if let Ok(log_file) = log_file {
                            let date_time = caps.name("dt").unwrap().as_str().to_string();
                            let dataset_filename =
                                Self::get_dataset_filename(matrices_count, date_time);
                            let dataset = OpenOptions::new().append(true).open(dataset_filename);
                            if let Ok(dataset_file) = dataset {
                                let log_reader = BufReader::new(&log_file);
                                let log_entries = log_reader
                                    .lines()
                                    .collect::<Result<HashSet<String>, _>>()
                                    .unwrap();

                                return Self {
                                    dataset_file,
                                    log_file,
                                    log_entries,
                                };
                            }
                        }
                    }
                }
            }
        }

        let dataset_filename = FileManager::get_dataset_filename(matrices_count, now.clone());
        let log_filename = FileManager::get_log_filename(matrices_count, now.clone());

        let dataset_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(dataset_filename)
            .unwrap();
        let log_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(log_filename)
            .unwrap();

        Self {
            dataset_file,
            log_file,
            log_entries: HashSet::new(),
        }
    }

    fn get_dataset_filename(count: usize, dt: String) -> String {
        format!("dataset_{}_{}.csv", count, dt)
    }

    fn get_log_filename(count: usize, dt: String) -> String {
        format!("log_{}_{}.csv", count, dt)
    }

    fn now() -> String {
        Local::now().format("%Y-%m-%d_%H_00").to_string()
    }

    fn get_log_file_path() -> Option<String> {
        let curr_dir = current_dir().unwrap();
        let files_list: Vec<DirEntry> = fs::read_dir(curr_dir)
            .unwrap()
            .filter_map(|file: Result<DirEntry, _>| {
                if let Ok(file) = file {
                    if file.file_name().into_string().unwrap().contains("log_") {
                        return Some(file);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            })
            .collect();

        if files_list.is_empty() {
            return None;
        } else if files_list.len() == 1 {
            return match files_list.into_iter().nth(0) {
                Some(entry) => Some(entry.path().to_str().unwrap().to_string()),
                None => None,
            };
        }

        let newest_file = files_list
            .into_iter()
            .max_by_key(|entry| {
                fs::metadata(entry.path())
                    .and_then(|metadata| metadata.modified())
                    .unwrap_or(UNIX_EPOCH)
            })
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .to_string();

        Some(newest_file)
    }
}
