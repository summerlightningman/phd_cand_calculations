use phd_cand::problems::travelling_salesman::algorithms::{
    ant_colony::algorithm::TSAntColonyAlgorithm, bee_colony::algorithm::TSBeeColonyAlgorithm,
    genetic::algorithm::TSGeneticAlgorithm,
};
use phd_cand::problems::travelling_salesman::solution::Solution;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug)]
pub struct TSSolution {
    pub solution: Solution,
}

impl Serialize for TSSolution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = if self.solution.time.is_some() { 4 } else { 3 };
        let mut s = serializer.serialize_struct("Solution", len)?;
        s.serialize_field("path", &self.solution.path)?;
        s.serialize_field("distance", &self.solution.distance)?;
        s.serialize_field("fitness", &self.solution.fitness)?;
        if let Some(time) = self.solution.time {
            s.serialize_field("time", &time)?;
        }
        s.end()
    }
}

fn map_solutions(
    calculation_result: Result<Vec<Solution>, &str>,
) -> Result<Vec<TSSolution>, String> {
    calculation_result
        .map(|solutions| {
            solutions
                .into_iter()
                .map(|solution| TSSolution { solution })
                .collect()
        })
        .map_err(|e| e.to_string())
}

pub trait OptimizationAlgorithm {
    fn calculate(&self) -> Result<Vec<TSSolution>, String>;
}

impl OptimizationAlgorithm for TSBeeColonyAlgorithm {
    fn calculate(&self) -> Result<Vec<TSSolution>, String> {
        map_solutions(self.run())
    }
}

impl OptimizationAlgorithm for TSAntColonyAlgorithm {
    fn calculate(&self) -> Result<Vec<TSSolution>, String> {
        map_solutions(self.run())
    }
}

impl OptimizationAlgorithm for TSGeneticAlgorithm {
    fn calculate(&self) -> Result<Vec<TSSolution>, String> {
        map_solutions(self.run())
    }
}
