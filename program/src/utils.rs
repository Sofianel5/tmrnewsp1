pub fn decode_embedding(hex: &str) -> Vec<f32> {
    // Remove the first two characters
    let data = &hex[2..];
    let mut embedding = Vec::new();

    // Iterate over the data in chunks of 8 characters
    for i in (0..data.len()).step_by(8) {
        let hex_chunk = &data[i..i + 8];
        let shifted = u64::from_str_radix(hex_chunk, 16).unwrap();
        embedding.push(shifted as f32 / 10f32.powi(9));
    }

    embedding
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product = a.iter().zip(b.iter()).map(|(a, b)| a * b).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (norm_a * norm_b)
}

pub fn compute_score(embedding: &[f32], truth: &[f32], value: u128) -> f32 {
    // Bound the similarity between 0 and 1
    let similarity = cosine_similarity(embedding, truth).clamp(0.0, 1.0);
    similarity * value as f32
}

pub fn score(encoded_embedding: &str, encoded_truth: &str, value: u128) -> f32 {
    let embedding = decode_embedding(encoded_embedding);
    let truth = decode_embedding(encoded_truth);
    compute_score(&embedding, &truth, value)
}