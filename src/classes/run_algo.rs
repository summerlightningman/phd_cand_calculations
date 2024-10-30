use std::cell::RefCell;
use phd_cand::algorithms::bee_colony::research_methods::{reverse_elements, swap_indexes};
use phd_cand::algorithms::genetic::methods::{Mutate, Select};
use phd_cand::problems::travelling_salesman::algorithms::{
    ant_colony::builder::TSAntColonyAlgorithmBuilder,
    bee_colony::builder::TSBeeColonyAlgorithmBuilder, genetic::builder::TSGeneticAlgorithmBuilder,
};
use phd_cand::problems::travelling_salesman::solution::Solution;
use crate::types::OptimizationAlgorithmEnum;

use super::algorithm_params::AlgorithmParams;

pub fn run_algo(params: AlgorithmParams, matrix: Vec<Vec<f64>>) -> Result<Vec<Solution>, String> {
    let algo = match params {
        AlgorithmParams::AC { alpha, beta, q, p } => {
            let algo = TSAntColonyAlgorithmBuilder::new(matrix)
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

            let algo = TSBeeColonyAlgorithmBuilder::new(matrix, func)
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
            let algo = TSGeneticAlgorithmBuilder::new(matrix, mutate, select)
                .p_mutation(p_mutation)
                .solutions_count(1).build();
            OptimizationAlgorithmEnum::GA(algo)
        }
    };

    struct CallBackInfo {
        attempts: u8,
        best_last_result: f64,
    }
    let info = RefCell::new(CallBackInfo { attempts: 0, best_last_result: -1.0 });

    let callback_fn = move |solutions: Vec<Solution>| {
        let mut callback_info = info.borrow_mut();
        let best_solution = solutions.first().unwrap();
        if best_solution.distance >= callback_info.best_last_result && callback_info.best_last_result > 0.0 {
            callback_info.attempts += 1;
        } else {
            callback_info.attempts = 0;
        }
        return if callback_info.attempts >= 5 {
            false
        } else {
            callback_info.best_last_result = best_solution.distance;
            true
        }
    };

    algo.calculate(callback_fn)
}
