use super::algorithm_params::AlgorithmParams;

pub const ALGORITHMS: [AlgorithmParams; 30] = [
    AlgorithmParams::AC {
        alpha: 1.0,
        beta: 2.0,
        q: 100.0,
        p: 0.5,
        actors_count: 50,
    },
    AlgorithmParams::AC {
        alpha: 2.0,
        beta: 1.0,
        q: 100.0,
        p: 0.5,
        actors_count: 50,
    },
    AlgorithmParams::AC {
        alpha: 1.0,
        beta: 5.0,
        q: 100.0,
        p: 0.5,
        actors_count: 50,
    },
    AlgorithmParams::AC {
        alpha: 1.0,
        beta: 2.0,
        q: 100.0,
        p: 0.1,
        actors_count: 50,
    },
    AlgorithmParams::AC {
        alpha: 1.0,
        beta: 2.0,
        q: 100.0,
        p: 0.01,
        actors_count: 50,
    },
    AlgorithmParams::AC {
        alpha: 1.0,
        beta: 2.0,
        q: 100.0,
        p: 0.5,
        actors_count: 20,
    },
    AlgorithmParams::AC {
        alpha: 1.0,
        beta: 2.0,
        q: 100.0,
        p: 0.5,
        actors_count: 100,
    },
    AlgorithmParams::AC {
        alpha: 1.0,
        beta: 2.0,
        q: 10.0,
        p: 0.5,
        actors_count: 50,
    },
    AlgorithmParams::AC {
        alpha: 1.0,
        beta: 2.0,
        q: 1000.0,
        p: 0.5,
        actors_count: 50,
    },
    AlgorithmParams::AC {
        alpha: 0.5,
        beta: 0.5,
        q: 100.0,
        p: 0.5,
        actors_count: 50,
    },
    AlgorithmParams::BC {
        workers_part: 0.5,
        research_func: "swap_indexes",
        actors_count: 50,
    },
    AlgorithmParams::BC {
        workers_part: 0.7,
        research_func: "swap_indexes",
        actors_count: 50,
    },
    AlgorithmParams::BC {
        workers_part: 0.3,
        research_func: "swap_indexes",
        actors_count: 50,
    },
    AlgorithmParams::BC {
        workers_part: 0.5,
        research_func: "reverse_elements",
        actors_count: 50,
    },
    AlgorithmParams::BC {
        workers_part: 0.5,
        research_func: "swap_indexes",
        actors_count: 20,
    },
    AlgorithmParams::BC {
        workers_part: 0.5,
        research_func: "swap_indexes",
        actors_count: 100,
    },
    AlgorithmParams::GA {
        p_mutation: 0.05,
        mutate_func: "swap_indexes",
        select_func: "tournament",
        actors_count: 100,
    },
    AlgorithmParams::GA {
        p_mutation: 0.05,
        mutate_func: "reverse_elements",
        select_func: "tournament",
        actors_count: 100,
    },
    AlgorithmParams::GA {
        p_mutation: 0.05,
        mutate_func: "swap_indexes",
        select_func: "roulette",
        actors_count: 100,
    },
    AlgorithmParams::GA {
        p_mutation: 0.05,
        mutate_func: "swap_indexes",
        select_func: "best_n",
        actors_count: 100,
    },
    AlgorithmParams::GA {
        p_mutation: 0.01,
        mutate_func: "swap_indexes",
        select_func: "tournament",
        actors_count: 100,
    },
    AlgorithmParams::GA {
        p_mutation: 0.1,
        mutate_func: "swap_indexes",
        select_func: "tournament",
        actors_count: 100,
    },
    AlgorithmParams::GA {
        p_mutation: 0.05,
        mutate_func: "swap_indexes",
        select_func: "tournament",
        actors_count: 50,
    },
    AlgorithmParams::GA {
        p_mutation: 0.05,
        mutate_func: "swap_indexes",
        select_func: "tournament",
        actors_count: 200,
    },
    AlgorithmParams::SA {
        initial_temperature: 1000.0,
        final_temperature: 1.0,
        cooling_rate: 0.95,
        mutate_func: "swap_indexes",
    },
    AlgorithmParams::SA {
        initial_temperature: 1000.0,
        final_temperature: 1.0,
        cooling_rate: 0.8,
        mutate_func: "swap_indexes",
    },
    AlgorithmParams::SA {
        initial_temperature: 500.0,
        final_temperature: 1.0,
        cooling_rate: 0.95,
        mutate_func: "swap_indexes",
    },
    AlgorithmParams::SA {
        initial_temperature: 500.0,
        final_temperature: 1.0,
        cooling_rate: 0.8,
        mutate_func: "swap_indexes",
    },
    AlgorithmParams::SA {
        initial_temperature: 1000.0,
        final_temperature: 1.0,
        cooling_rate: 0.95,
        mutate_func: "reverse_elements",
    },
    AlgorithmParams::SA {
        initial_temperature: 1000.0,
        final_temperature: 1.0,
        cooling_rate: 0.99,
        mutate_func: "swap_indexes",
    },
];

