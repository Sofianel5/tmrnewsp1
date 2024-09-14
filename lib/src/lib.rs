use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PredictionCommitment {
    pub encoded_embedding: String,
    pub value: u128
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofInput {
    pub predictions: Vec<PredictionCommitment>,
    pub truth: PredictionCommitment
}