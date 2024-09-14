//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use tiny_keccak::{Hasher, Sha3};
use lib::ProofInput;
mod utils;
use utils::score;

pub fn main() {
    let input = sp1_zkvm::io::read::<ProofInput>();

    // Compute state root as iterated hash of embeddings and values
    let mut state_root = [0u8; 32];
    for prediction in &input.predictions {
        let mut hasher = Sha3::v256();
        hasher.update(&state_root);
        hasher.update(prediction.encoded_embedding.as_bytes());
        hasher.update(&prediction.value.to_le_bytes());
        hasher.finalize(&mut state_root);
    }

    sp1_zkvm::io::commit(&state_root);

    // Then, compute the total score
    let total_score: f32 = input.predictions.iter().map(|prediction| {
        let encoded_embedding = &prediction.encoded_embedding;
        let encoded_truth = &input.truth.encoded_embedding;
        let value = prediction.value;
        score(encoded_embedding, encoded_truth, value)
    }).sum();

    sp1_zkvm::io::commit(&total_score.to_le_bytes());
}
