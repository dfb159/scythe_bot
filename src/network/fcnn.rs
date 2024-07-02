use ndarray::{Array1, Array2};

type MathFn = fn(f64) -> f64;
type MathFnClosure<'a> = Box<dyn Fn(f64) -> f64 + 'a>;

pub(crate) enum MLFunction<'a> {
    Linear,
    Sigmoid,
    Tanh,
    ReLU,
    LeakyReLU,
    ELU,
    Custom(MathFnClosure<'a>, MathFnClosure<'a>), // Custom function and derivative
}

impl<'a> MLFunction<'a> {
    fn as_fn_pair(&self) -> (MathFnClosure, MathFnClosure) {
        match self {
            MLFunction::Linear => (Box::new(|x| x), Box::new(|_| 1.0)),
            MLFunction::Sigmoid => (
                Box::new(sigmoid),
                Box::new(|x| {
                    let s = sigmoid(x);
                    s * (1.0 - s)
                }),
            ),
            MLFunction::Tanh => (
                Box::new(|x| tanh_norm(x)),
                Box::new(|x| 0.5 - x.tanh().powi(2) / 2.0),
            ),
            MLFunction::ReLU => (
                Box::new(relu),
                Box::new(|x| if x > 0.0 { 1.0 } else { 0.0 }),
            ),
            MLFunction::LeakyReLU => (
                Box::new(leaky_relu),
                Box::new(|x| if x > 0.0 { 1.0 } else { 0.01 }),
            ),
            MLFunction::ELU => (
                Box::new(elu),
                Box::new(|x| if x > 0.0 { 1.0 } else { x.exp() }),
            ),
            MLFunction::Custom(f, df) => (Box::new(f), Box::new(df)),
        }
    }
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn tanh_norm(x: f64) -> f64 {
    x.tanh() / 2.0 + 0.5
}

fn relu(x: f64) -> f64 {
    x.max(0.0)
}

fn leaky_relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.01 * x
    }
}

fn elu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        x.exp() - 1.0
    }
}

pub(crate) struct FCNN<'a> {
    weights: Vec<Array2<f64>>,
    biases: Vec<Array1<f64>>,
    functions: Vec<&'a MLFunction<'a>>,
    heights: Vec<usize>,
}

impl<'a> FCNN<'a> {
    pub fn new(heights: Vec<usize>, output_func: &'a MLFunction<'a>) -> Self {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let mut functions = Vec::new();
        for i in 0..heights.len() - 1 {
            weights.push(Array2::zeros((heights[i + 1], heights[i])));
            biases.push(Array1::zeros(heights[i + 1]));
            functions.push(if i < heights.len() - 1 {
                &MLFunction::ReLU
            } else {
                &output_func
            });
        }
        Self {
            weights,
            biases,
            functions,
            heights,
        }
    }

    pub fn new_softmax(heights: Vec<usize>) -> Self {
        Self::new(heights, &MLFunction::Tanh)
    }
}

pub(crate) trait Predictor {
    fn predict(&self, input: &Array1<f64>) -> Array1<f64>;
}

impl Predictor for FCNN<'_> {
    fn predict(&self, input: &Array1<f64>) -> Array1<f64> {
        if input.len() != self.heights[0] {
            panic!("Input size does not match the input layer size!");
        }
        let mut result = input.clone();
        for i in 0..self.heights.len() - 1 {
            result = self.weights[i].dot(&result) + &self.biases[i];
            let (f, _) = self.functions[i].as_fn_pair();
            result.mapv_inplace(f);
        }
        1.0 / result.sum() * result // probability
    }
}

pub(crate) trait Learner {
    fn learn(&mut self, input: &Array1<f64>, target: &Array1<f64>, learning_rate: f64);
}

impl Learner for FCNN<'_> {
    fn learn(&mut self, input: &Array1<f64>, target: &Array1<f64>, learning_rate: f64) {
        if input.len() != self.heights[0] || target.len() != self.heights[self.heights.len() - 1] {
            panic!("Input or target size does not match the network architecture!");
        }

        let mut outputs = Vec::new();
        let mut derivs = Vec::new();
        outputs.push(input.clone());
        for i in 0..self.heights.len() - 1 {
            let w_sum = self.weights[i].dot(&outputs[i]) + &self.biases[i];
            let (f, df) = self.functions[i].as_fn_pair();
            outputs.push(w_sum.mapv(f));
            derivs.push(w_sum.mapv(df));
        }

        let mut delta = target - &outputs[outputs.len() - 1];
        for i in (0..self.heights.len() - 1).rev() {
            let inner_delta = &delta / &derivs[i]; // deriv of the nth hidden layer
            self.biases[i] = &self.biases[i] + &inner_delta * learning_rate;

            delta.map_inplace(|x| *x = 0.0); // reset delta
            let (J, K) = self.weights[i].dim();
            for j in 0..J {
                for k in 0..K {
                    delta[j] += inner_delta[j] / self.weights[i][(j, k)];
                    self.weights[i][(j, k)] += inner_delta[j] / outputs[i][k] * learning_rate;
                }
            }

            delta = delta / self.heights[i] as f64; // renormalize delta with height of layer i-1. This approximates the inverse matrix of the weights.
        }
    }
}
