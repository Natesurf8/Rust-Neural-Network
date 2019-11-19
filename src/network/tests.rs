use std::rc::Rc;
use std::cell::RefCell;

use super::execution_sequence::ActivationSequence;
use super::operator::Operator;
use super::placeholder::Placeholder;

// tests the network code by making a network that just squares its inputs
struct SqOperator {
    inputs: Vec<Rc<RefCell<Placeholder>>>,
    outputs: Vec<Rc<RefCell<Placeholder>>>,
}
impl SqOperator {
    fn new(inputs: Vec<Rc<RefCell<Placeholder>>>, outputs: Vec<Rc<RefCell<Placeholder>>>) -> SqOperator {
        assert!(inputs.len() == 1 && outputs.len() == 1, "wrong connections");
        assert!(inputs[0].borrow().get_data().len() == outputs[0].borrow().get_data().len(), "mismatched connections");
        SqOperator {
            inputs: inputs,
            outputs: outputs,
        }
    }

}
impl Operator for SqOperator {
    fn activate(&self) {
        let a = self.inputs[0].borrow();
        let a = a.get_data();

        let mut b = self.outputs[0].borrow_mut();
        let mut b = b.get_data_mut();

        for i in 0..b.len() {
            b[i] = a[i] * a[i];
        }
    }
}


struct SqNetwork {
    operators: Vec<Rc<dyn Operator>>,
    placeholders: Vec<Rc<RefCell<Placeholder>>>,

    squares: Rc<RefCell<Placeholder>>,
    square_sequence: ActivationSequence,
}
impl SqNetwork {
    pub fn new() -> SqNetwork {

        // the nodes in the network
        let inp = Rc::new(RefCell::new(Placeholder::new("in", vec![2,3])));
        let out = Rc::new(RefCell::new(Placeholder::new("out", vec![2,3])));
        let squarer = Rc::new(SqOperator::new(vec![inp.clone()],vec![out.clone()]));

        SqNetwork {
            placeholders: vec![inp.clone(), out.clone()],
            operators: vec![squarer.clone()],
            squares: out.clone(),
            square_sequence: ActivationSequence {
                    operators: vec![squarer.clone()],
                    inputs: vec![inp.clone()],
            },
        }
    }

    fn get_squares(&mut self, values: &[f64]) -> Vec<f64> {
        self.square_sequence.execute(&vec![values]);
        return self.squares.borrow().get_data_clone();
    }
}

// tests the network code by creating a network that squares a list of numbers
#[test]
fn test_sq_network() {
    println!("beginning SqNetwork test");

    let mut sqnet = SqNetwork::new();

    let test_input = vec![1.0,2.0,3.0,2.0,5.0,4.0];
    let expected_output = vec![1.0,4.0,9.0,4.0,25.0,16.0];

    println!("squaring values . . .");
    let output = sqnet.get_squares(&test_input);

    println!("sqnet results: ");
    println!("input: {:?}", test_input);
    println!("output: {:?}", output);
    println!("expected_output: {:?}", expected_output);
}
