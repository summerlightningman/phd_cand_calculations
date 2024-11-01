use phd_cand::problems::travelling_salesman::algorithms::{
    ant_colony::algorithm::TSAntColonyAlgorithm, bee_colony::algorithm::TSBeeColonyAlgorithm,
    genetic::algorithm::TSGeneticAlgorithm,
};
use phd_cand::problems::travelling_salesman::solution::Solution;
use super::classes::run_algo::RunAlgoResult;

pub enum OptimizationAlgorithmEnum {
    BC(TSBeeColonyAlgorithm),
    AC(TSAntColonyAlgorithm),
    GA(TSGeneticAlgorithm)
}

impl OptimizationAlgorithmEnum {
    pub fn calculate<F>(&self, callback_fn: F) -> Result<Vec<Solution>, String>
    where
        F: Fn(Vec<Solution>) -> bool {
        match self {
            OptimizationAlgorithmEnum::BC(algo) => algo.run(callback_fn),
            OptimizationAlgorithmEnum::AC(algo) => algo.run(callback_fn),
            OptimizationAlgorithmEnum::GA(algo) => algo.run(callback_fn),
        }
    }
}

pub struct FileRow(pub String);

pub enum SenderInfo {
    DatasetRow(RunAlgoResult),
    FileRow(FileRow),
}
