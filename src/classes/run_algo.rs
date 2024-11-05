use crate::types::OptimizationAlgorithmEnum;
use phd_cand::algorithms::bee_colony::research_methods::{reverse_elements, swap_indexes};
use phd_cand::algorithms::genetic::methods::{Mutate, Select};
use phd_cand::problems::travelling_salesman::algorithms::{
    ant_colony::builder::TSAntColonyAlgorithmBuilder,
    bee_colony::builder::TSBeeColonyAlgorithmBuilder, genetic::builder::TSGeneticAlgorithmBuilder,
};
use phd_cand::problems::travelling_salesman::solution::Solution;
use phd_cand::problems::travelling_salesman::types::Matrix;
use std::cell::RefCell;
use std::time::Instant;
use super::algorithm_params::AlgorithmParams;

use serde_json;

use serde::{Serialize, Serializer};

fn as_json<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: Serialize,
    S: Serializer,
{
    let json_string = serde_json::to_string(value).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&json_string)
}


#[derive(Clone, Serialize)]
pub struct RunAlgoResultIteration {
    iter_num: usize,
    calc_time: u128,
    distance: f64,
    time: usize,
    fitness: f32,
}

#[derive(Clone, Serialize)]
pub struct RunAlgoResult {
    #[serde(serialize_with = "as_json")]
    pub matrix: Matrix,
    #[serde(serialize_with = "as_json")]
    pub algo: AlgorithmParams,
    #[serde(serialize_with = "as_json")]
    pub iterations: Vec<RunAlgoResultIteration>,
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
                .solutions_count(1)
                .build();
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
                .solutions_count(1)
                .build();
            OptimizationAlgorithmEnum::GA(algo)
        }
    };

    const MAX_ATTEMPTS: usize = 5;
    let iterations: RefCell<Vec<RunAlgoResultIteration>> = RefCell::new(Vec::with_capacity(60));

    let calculation_start = RefCell::new(Instant::now());
    let callback_fn = |solutions: Vec<Solution>| {
        let best_solution = solutions.first().unwrap();
        let mut iters = iterations.borrow_mut();

        let result = RunAlgoResultIteration {
            iter_num: iters.len() + 1,
            calc_time: calculation_start.borrow().elapsed().as_millis(),
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
                    calculation_start.replace(Instant::now());
                    return false;
                }
            }
        }

        calculation_start.replace(Instant::now());
        return true;
    };

    if let Err(_) = algo.calculate(callback_fn) {
        return Err("Calculation Error".to_string());
    }

    let info_cell = RunAlgoResult {
        matrix,
        algo: params,
        iterations: iterations.into_inner(),
    };

    Ok(info_cell)
}
