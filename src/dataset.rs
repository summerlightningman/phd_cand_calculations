use crate::classes::algorithm_params::AlgorithmParams;
use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone)]
pub struct Matrix(Vec<Vec<f64>>);

#[derive(Debug)]
pub struct DatasetRow {
    pub algo: AlgorithmParams,
    pub matrix: Matrix,
    pub fitness: f32,
    pub file_name: String,
}

impl DatasetRow {
    pub fn new(
        file_name: String,
        algo: AlgorithmParams,
        matrix: Vec<Vec<f64>>,
        fitness: f32,
    ) -> Self {
        Self {
            file_name,
            algo,
            matrix: Matrix(matrix),
            fitness,
        }
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut result = String::from("[");
        for (i, row) in self.0.iter().enumerate() {
            result.push('[');
            for (j, &val) in row.iter().enumerate() {
                result.push_str(&val.to_string());
                if j != row.len() - 1 {
                    result.push(',');
                }
            }
            result.push(']');
            if i != self.0.len() - 1 {
                result.push_str(",");
            }
        }
        result.push(']');
        write!(f, "{}", result)
    }
}

impl Serialize for Matrix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for row in &self.0 {
            seq.serialize_element(row)?;
        }
        seq.end()
    }
}

// Сериализация DatasetRow, включая сериализацию algo как JSON
impl Serialize for DatasetRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("DatasetRow", 3)?;

        let algo_json = serde_json::to_string(&self.algo)
            .unwrap()
            .replace("\"\"", "\"");
        s.serialize_field("algo", &algo_json)?;

        let matrix_str = format!("{}", self.matrix); // Если хотите строку в CSV
        s.serialize_field("matrix", &matrix_str)?;

        s.serialize_field("fitness", &self.fitness)?;

        s.end()
    }
}
