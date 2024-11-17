use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone, Debug)]
pub enum AlgorithmParams {
    AC {
        alpha: f64,
        beta: f64,
        q: f64,
        p: f64,
        actors_count: usize,
    },
    BC {
        workers_part: f32,
        research_func: &'static str,
        actors_count: usize,
    },
    GA {
        p_mutation: f32,
        select_func: &'static str,
        mutate_func: &'static str,
        actors_count: usize,
    },
    SA {
        initial_temperature: usize,
        final_temperature: usize,
        cooling_rate: f32,
        mutate_func: &'static str
    }
}

impl Serialize for AlgorithmParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AlgorithmParams::AC { alpha, beta, q, p, actors_count } => {
                let mut s = serializer.serialize_struct("AC", 6)?;
                s.serialize_field("type", "AC")?;
                s.serialize_field("actors_count", actors_count)?;
                s.serialize_field("alpha", alpha)?;
                s.serialize_field("beta", beta)?;
                s.serialize_field("q", q)?;
                s.serialize_field("p", p)?;
                s.end()
            }
            AlgorithmParams::BC {
                workers_part,
                research_func,
                actors_count
            } => {
                let mut s = serializer.serialize_struct("BC", 4)?;
                s.serialize_field("type", "BC")?;
                s.serialize_field("actors_count", actors_count)?;
                s.serialize_field("workers_part", workers_part)?;
                s.serialize_field("regenerate_func", research_func)?;
                s.end()
            }
            AlgorithmParams::GA {
                p_mutation,
                select_func,
                mutate_func,
                actors_count
            } => {
                let mut s = serializer.serialize_struct("GA", 5)?;
                s.serialize_field("type", "GA")?;
                s.serialize_field("actors_count", actors_count)?;
                s.serialize_field("p_mutation", p_mutation)?;
                s.serialize_field("select_func", select_func)?;
                s.serialize_field("mutate_func", mutate_func)?;
                s.end()
            },
            AlgorithmParams::SA {
                initial_temperature,
                final_temperature,
                cooling_rate,
                mutate_func
            } => {
                let mut s = serializer.serialize_struct("SA", 5)?;
                s.serialize_field("type", "SA")?;
                s.serialize_field("mutate_func", mutate_func)?;
                s.serialize_field("cooling_rate", cooling_rate)?;
                s.serialize_field("initial_temperature", initial_temperature)?;
                s.serialize_field("final_temperature", final_temperature)?;
            }
        }
    }
}
