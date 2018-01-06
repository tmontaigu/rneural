extern crate rand;

mod neuron;
mod layer;
mod matrix;

use neuron::Neuron;
use matrix::Matrix;

fn main() {
    println!("Hello, world!");
    let n = Neuron::new(90.0);

    println!("{}", n);

    let mat = Matrix::new_randomized(4,2);
    println!("{}", mat);

    let mat_t = mat.transposed();
    println!("{}", mat_t);

}
