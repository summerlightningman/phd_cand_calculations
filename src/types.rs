use super::classes::run_algo::RunAlgoResult;

pub struct FileRow(pub String);

pub enum SenderInfo {
    DatasetRow(RunAlgoResult),
    FileRow(FileRow),
}
