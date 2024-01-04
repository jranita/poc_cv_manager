pub mod client_company;
pub mod cv;
pub mod job_function;
pub mod keyword;
pub mod user;

use serde::{Deserialize, Serialize};

pub fn number_vec_to_string(vector: &[i32]) -> String {
    return "{".to_owned()
        + &vector
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
        + "}";
}
