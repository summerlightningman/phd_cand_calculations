use std::panic::{self, UnwindSafe};

use phd_cand::algorithms::bee_colony::research_methods::{reverse_elements, swap_indexes};
use phd_cand::algorithms::genetic::methods::{Mutate, Select};
use phd_cand::problems::travelling_salesman::algorithms::{
    ant_colony::builder::TSAntColonyAlgorithmBuilder,
    bee_colony::builder::TSBeeColonyAlgorithmBuilder, genetic::builder::TSGeneticAlgorithmBuilder,
};

use crate::types::{OptimizationAlgorithm, TSSolution};

use super::algorithm_params::AlgorithmParams;

pub fn run_algo(params: AlgorithmParams, matrix: Vec<Vec<f64>>) -> Result<Vec<TSSolution>, String> {
    let algo: Box<dyn OptimizationAlgorithm> = match params {
        AlgorithmParams::AC { alpha, beta, q, p } => {
            let algo = TSAntColonyAlgorithmBuilder::new(matrix)
                .alpha(alpha)
                .beta(beta)
                .q(q)
                .p(p)
                .solutions_count(1);
            Box::new(algo.build())
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
                .solutions_count(1);
            Box::new(algo.build())
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
                .solutions_count(1);
            Box::new(algo.build())
        }
    };

    algo.calculate()
}
