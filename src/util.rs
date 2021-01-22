pub fn add_matrix_with_probability(
    matrix_a: &mut [f64],
    matrix_b: &[f64],
    probability: f64,
) {
    for i in 0..matrix_a.len() {
        matrix_a[i] += matrix_b[i] * probability;
    }
}
