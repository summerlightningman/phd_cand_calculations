use phd_cand_algorithms::types::{Individual, Matrix, Rule, Task};
use phd_cand_algorithms::builders::{
    BeeColonyAlgorithmBuilder,
    SimulatedAnnealingBuilder,
    AntColonyAlgorithmBuilder,
    GeneticAlgorithmBuilder
};

use std::cell::RefCell;
use std::collections::HashMap;
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
    results: HashMap<String, f64>,
    path: Vec<usize>,
    weight: f32,
}

#[derive(Clone, Serialize)]
pub struct RunAlgoResult {
    #[serde(serialize_with = "as_json")]
    pub tasks: Vec<Task>,
    #[serde(serialize_with = "as_json")]
    pub algo: AlgorithmParams,
    #[serde(serialize_with = "as_json")]
    pub iterations: Vec<RunAlgoResultIteration>,
}

pub fn run_algo(params: AlgorithmParams, tasks: Vec<Task>, rules: Vec<Rule>) -> Option<RunAlgoResult> {
    const MAX_ATTEMPTS: usize = 5;
    let iterations: RefCell<Vec<RunAlgoResultIteration>> = RefCell::new(Vec::with_capacity(60));
    let calculation_start = RefCell::new(Instant::now());
    let callback_fn = |individuals: Vec<Individual>| {
        let best_solution = individuals.first().unwrap();
        let mut iters = iterations.borrow_mut();

        match best_solution.weight {
            Some(weight) => {
                let result = RunAlgoResultIteration {
                    iter_num: iters.len() + 1,
                    calc_time: calculation_start.borrow().elapsed().as_millis(),
                    path: best_solution.value.clone(),
                    results: best_solution.results.clone(),
                    weight
                };
                iters.push(result);
            },
            None => {
                return false
            }
        }

        if iters.len() >= MAX_ATTEMPTS {
            let mut attempts: usize = 0;
            for i in 1..iters.len() {
                if iters[i].weight >= iters[i - 1].weight {
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

    let tasks_clone = tasks.clone();
    let result = match params {
        AlgorithmParams::AC { alpha, beta, q, p, actors_count } => {
            let algo = AntColonyAlgorithmBuilder::new(tasks_clone)
                .rules(rules)
                .actors_count(actors_count)
                .alpha(alpha)
                .beta(beta)
                .q(q)
                .p(p)
                .solutions_count(1)
                .build();
            algo.run(callback_fn)
        },
        AlgorithmParams::BC {
            workers_part,
            research_func,
            actors_count,
        } => {
            let algo = BeeColonyAlgorithmBuilder::new(tasks_clone)
                .workers_part(workers_part)
                .solutions_count(1)
                .research_func_str(research_func)
                .actors_count(actors_count)
                .build();
            algo.run(callback_fn)
        },
        AlgorithmParams::GA {
            p_mutation,
            select_func,
            mutate_func,
            actors_count
        } => {
            let algo = GeneticAlgorithmBuilder::new(tasks_clone)
                .p_mutation(p_mutation)
                .select_func_str(select_func)
                .mutate_func_str(mutate_func)
                .actors_count(actors_count)
                .build();
            algo.run(callback_fn)
        },
        AlgorithmParams::SA {
            initial_temperature,
            final_temperature,
            cooling_rate,
            mutate_func
        } => {
            let algo = SimulatedAnnealingBuilder::new(tasks_clone)
                .initial_temperature(initial_temperature)
                .final_temperature(final_temperature)
                .cooling_rate(cooling_rate)
                .mutate_func_str(mutate_func)
                .build();
            algo.run(callback_fn)
        }
    };

    if let Ok(_) = result {
        Some(RunAlgoResult {
            tasks,
            algo,
            iterations: iterations.into(),
        })
    } else {
        None
    }
}
