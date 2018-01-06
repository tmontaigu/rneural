use neuron::Neuron;

pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(size: usize) -> Self {
        let mut neurons = Vec::<Neuron>::with_capacity(size);
        for _ in 0..size {
            neurons.push(Neuron::new(0.0));
        }
        Layer {neurons: neurons }
    }
}


