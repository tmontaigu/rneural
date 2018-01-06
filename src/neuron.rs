use std::fmt;


#[derive(Debug)]
pub struct Neuron {
    value: f64,
    activated_value: f64,
    derived_value: f64,
}

impl Neuron {

    pub fn new(value: f64) -> Self {
       let mut n = Neuron {value: value, activated_value: 0.0, derived_value: 0.0};
        n.activate();
        n.derive();
        n
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn activated_value(&self) -> f64 {
        self.activated_value
    }

    pub fn derived_value(&self) -> f64 {
        self.derived_value
    }


    pub fn activate(&mut self) {
        self.activated_value = fast_sigmoid(self.value);
    }

    pub fn derive(&mut self) {
        self.derived_value = fast_sigmoid(self.activated_value) * (1.0 - fast_sigmoid(self.activated_value));
    }
}

fn fast_sigmoid(x: f64) -> f64 {
    x / (1.0 + x.abs())
}

impl fmt::Display for Neuron {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Neuron: val: {}, activated: {}, derived: {}",
         self.value, self.activated_value, self.derived_value)
    }
}