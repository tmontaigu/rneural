use layer::Layer;
use matrix::Matrix;

pub #[derive(Debug)]
struct NeuralNetwork {
    layers: Vec<Layer>,
    weight_matrices: Vec<Matrix>;
}

impl NeuralNetwork {
    pub fn new(topology: Vec<u32>) {
        let mut layers = Vec::<Layer>::with_capacity(topology.len());
        for i in 0..topology.len() {
            layers.push(Layer::new(topology[i] as usize));
        }

        let num_weight_mat = topology.len() - 1;
        let mut weight_matrices = Vec::<Matrix>::with_capacity(num_weight_mat);
        for i in 0..num_weight_mat {
            weight_matrices.push(Matrix::new_randomized(topology[i], topology[i + 1]));
        }

        NeuralNetwork{layers: layers, weight_matrices: weight_matrices}
    }
}