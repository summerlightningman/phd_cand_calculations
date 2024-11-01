use std::cell::RefCell;
use chrono::Local;
use phd_cand::algorithms::bee_colony::research_methods::{reverse_elements, swap_indexes};
use phd_cand::algorithms::genetic::methods::{Mutate, Select};
use phd_cand::problems::travelling_salesman::algorithms::{
    ant_colony::builder::TSAntColonyAlgorithmBuilder,
    bee_colony::builder::TSBeeColonyAlgorithmBuilder, genetic::builder::TSGeneticAlgorithmBuilder,
};
use phd_cand::problems::travelling_salesman::solution::Solution;
use phd_cand::problems::travelling_salesman::types::Matrix;
use crate::types::OptimizationAlgorithmEnum;

use super::algorithm_params::AlgorithmParams;
use serde_json;
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Clone)]
pub struct RunAlgoResultIteration {
    iter_num: usize,
    calc_time: i64,
    distance: f64,
    time: usize,
    fitness: f32,
}

impl Serialize for RunAlgoResultIteration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RunAlgoResultIteration", 5)?;
        state.serialize_field("iter_num", &self.iter_num)?;
        state.serialize_field("calc_time", &self.calc_time)?;
        state.serialize_field("distance", &self.distance)?;
        state.serialize_field("time", &self.time)?;
        state.serialize_field("fitness", &self.fitness)?;
        state.end()
    }
}

#[derive(Clone)]
pub struct RunAlgoResult {
    matrix: String,
    algo: String,
    iterations: String,
}

impl Serialize for RunAlgoResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("RunAlgoResult", 3)?;
        state.serialize_field("matrix", &self.matrix)?;
        state.serialize_field("algo", &serde_json::to_string(&self.algo).unwrap())?;
        state.serialize_field("iterations", &serde_json::to_string(&self.iterations).unwrap())?;
        state.end()
    }
}

pub fn run_algo(params: AlgorithmParams, matrix: Matrix) -> Result<RunAlgoResult, String> {
    let algo = match params {
        AlgorithmParams::AC { alpha, beta, q, p } => {
            let algo = TSAntColonyAlgorithmBuilder::new(matrix.clone())
                .alpha(alpha)
                .beta(beta)
                .q(q)
                .p(p)
                .solutions_count(1)
                .build();
            OptimizationAlgorithmEnum::AC(algo)
        }
        AlgorithmParams::BC {
            workers_part,
            regenerate_func,
        } => {
            let func = if regenerate_func.eq("swap_indexes") {
                swap_indexes(None)
            } else {
                reverse_elements(None)
            };

            let algo = TSBeeColonyAlgorithmBuilder::new(matrix.clone(), func)
                .workers_part(workers_part)
                .solutions_count(1).build();
            OptimizationAlgorithmEnum::BC(algo)
        }
        AlgorithmParams::GA {
            p_mutation,
            select_func,
            mutate_func,
        } => {
            let mutate = match mutate_func {
                "reverse" => Mutate::reverse_elements(None),
                _ => Mutate::swap_indexes(None),
            };
            let select = match select_func {
                "tournament" => Select::tournament(3, None),
                "roulette" => Select::roulette(None),
                _ => Select::stochastic(None),
            };
            let algo = TSGeneticAlgorithmBuilder::new(matrix.clone(), mutate, select)
                .p_mutation(p_mutation)
                .solutions_count(1).build();
            OptimizationAlgorithmEnum::GA(algo)
        }
    };

    const MAX_ATTEMPTS: usize = 5;

    let iterations: RefCell<Vec<RunAlgoResultIteration>> = RefCell::new(vec![]);

    let calculation_start = RefCell::new(Local::now());
    let callback_fn = |solutions: Vec<Solution>| {
        let best_solution = solutions.first().unwrap();
        let mut iters = iterations.borrow_mut();

        let result = RunAlgoResultIteration {
            iter_num: iters.len() + 1,
            calc_time: Local::now().signed_duration_since(*calculation_start.borrow()).num_milliseconds(),
            distance: best_solution.distance,
            time: best_solution.time.unwrap_or(0usize),
            fitness: best_solution.fitness,
        };
        iters.push(result);

        if iters.len() >= MAX_ATTEMPTS {
            let mut attempts: usize = 0;
            for i in 1..iters.len() {
                if iters[i].fitness >= iters[i - 1].fitness {
                    attempts += 1;
                } else {
                    attempts = 0;
                }

                if attempts >= MAX_ATTEMPTS {
                    calculation_start.replace(Local::now());
                    return false
                }
            }
        }

        calculation_start.replace(Local::now());
        return true
    };

    if let Err(_) = algo.calculate(callback_fn) {
        return Err("Calculation Error".to_string());
    }

    let mt = match serde_json::to_string(&matrix) {
        Ok(mt) => mt,
        Err(_) => return Err(format!("Cannot serialize matrix: {:?}", matrix))
    };
    let algorithm = match serde_json::to_string(&params) {
        Ok(par) => par,
        Err(_) => return Err(format!("Cannot serialize params: {:?}", params))
    };
    let iters = match serde_json::to_string(&iterations.take()) {
        Ok(iters) => iters,
        Err(_) => return Err(format!("Cannot serialize iterations: {:?}", params))
    };

    let info_cell = RunAlgoResult {
        matrix: mt,
        algo: algorithm,
        iterations: iters,
    };

    Ok(info_cell)
}
