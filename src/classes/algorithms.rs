use super::algorithm_params::AlgorithmParams;

pub const ALGORITHMS: [AlgorithmParams; 24] = [
    // AC variants
    AlgorithmParams::AC {
        alpha: 1.25,
        beta: 0.5,
        q: 0.75,
        p: 1.0,
    }, //
    AlgorithmParams::AC {
        alpha: 0.75,
        beta: 1.0,
        q: 1.25,
        p: 0.5,
    }, //
    AlgorithmParams::AC {
        alpha: 1.5,
        beta: 0.5,
        q: 0.75,
        p: 0.5,
    }, //
    AlgorithmParams::AC {
        alpha: 1.25,
        beta: 0.5,
        q: 1.0,
        p: 0.75,
    }, //
    AlgorithmParams::AC {
        alpha: 1.5,
        beta: 0.5,
        q: 1.0,
        p: 0.75,
    }, //
    AlgorithmParams::AC {
        alpha: 1.5,
        beta: 0.75,
        q: 1.25,
        p: 0.75,
    },
    AlgorithmParams::AC {
        alpha: 1.5,
        beta: 1.5,
        q: 1.25,
        p: 1.25,
    }, //
    AlgorithmParams::AC {
        alpha: 1.5,
        beta: 1.25,
        q: 1.0,
        p: 0.75,
    }, //
    // BC variants
    AlgorithmParams::BC {
        workers_part: 0.15,
        regenerate_func: "reverse_elements",
    },
    AlgorithmParams::BC {
        workers_part: 0.25,
        regenerate_func: "reverse_elements",
    },
    AlgorithmParams::BC {
        workers_part: 0.5,
        regenerate_func: "swap_indexes",
    },
    AlgorithmParams::BC {
        workers_part: 0.6,
        regenerate_func: "reverse_elements",
    },
    AlgorithmParams::BC {
        workers_part: 0.65,
        regenerate_func: "reverse_elements",
    },
    AlgorithmParams::BC {
        workers_part: 0.75,
        regenerate_func: "swap_indexes",
    },
    AlgorithmParams::BC {
        workers_part: 0.75,
        regenerate_func: "reverse_elements",
    },
    AlgorithmParams::BC {
        workers_part: 0.9,
        regenerate_func: "reverse_elements",
    },
    // GA variants
    AlgorithmParams::GA {
        p_mutation: 0.2,
        select_func: "roulette",
        mutate_func: "reverse_elements",
    }, //
    AlgorithmParams::GA {
        p_mutation: 0.2,
        select_func: "stochastic",
        mutate_func: "reverse_elements",
    },
    AlgorithmParams::GA {
        p_mutation: 0.2,
        select_func: "stochastic",
        mutate_func: "swap_indexes",
    },
    AlgorithmParams::GA {
        p_mutation: 0.2,
        select_func: "tournament",
        mutate_func: "reverse_elements",
    },
    AlgorithmParams::GA {
        p_mutation: 0.4,
        select_func: "tournament",
        mutate_func: "swap_indexes",
    }, //
    AlgorithmParams::GA {
        p_mutation: 0.4,
        select_func: "tournament",
        mutate_func: "reverse_elements",
    }, //
    AlgorithmParams::GA {
        p_mutation: 0.6,
        select_func: "tournament",
        mutate_func: "swap_indexes",
    }, //
    AlgorithmParams::GA {
        p_mutation: 0.6,
        select_func: "tournament",
        mutate_func: "reverse_elements",
    }, //
];
