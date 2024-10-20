use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone, Debug)]
pub enum AlgorithmParams {
    AC {
        alpha: f64,
        beta: f64,
        q: f64,
        p: f64,
    },
    BC {
        workers_part: f32,
        regenerate_func: &'static str,
    },
    GA {
        p_mutation: f32,
        select_func: &'static str,
        mutate_func: &'static str,
    },
}

impl Serialize for AlgorithmParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            AlgorithmParams::AC { alpha, beta, q, p } => {
                let mut s = serializer.serialize_struct("AC", 5)?;
                s.serialize_field("type", "AC")?;
                s.serialize_field("alpha", alpha)?;
                s.serialize_field("beta", beta)?;
                s.serialize_field("q", q)?;
                s.serialize_field("p", p)?;
                s.end()
            }
            AlgorithmParams::BC {
                workers_part,
                regenerate_func,
            } => {
                let mut s = serializer.serialize_struct("BC", 3)?;
                s.serialize_field("type", "BC")?;
                s.serialize_field("workers_part", workers_part)?;
                s.serialize_field("regenerate_func", regenerate_func)?;
                s.end()
            }
            AlgorithmParams::GA {
                p_mutation,
                select_func,
                mutate_func,
            } => {
                let mut s = serializer.serialize_struct("GA", 4)?;
                s.serialize_field("type", "GA")?;
                s.serialize_field("p_mutation", p_mutation)?;
                s.serialize_field("select_func", select_func)?;
                s.serialize_field("mutate_func", mutate_func)?;
                s.end()
            }
        }
    }
}
