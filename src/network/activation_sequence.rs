use super::placeholder::Placeholder;
use super::operator::Operator;
use std::cell::RefCell;
use std::rc::Rc;

/// An execution sequence takes a list of data, copies them to the appropriate placeholders, and activates its sequence of operators
pub struct ActivationSequence {
    pub operators: Vec<Rc<dyn Operator>>,
    pub inputs: Vec<Rc<RefCell<Placeholder>>>,
}
impl ActivationSequence {
    pub fn activate(&self, input: &[&[f64]]) {
        assert!(input.len() == self.inputs.len());
        for i in 0..self.inputs.len() {
            self.inputs[i].borrow_mut().copy_data(input[i]);
        }

        for op in &self.operators {
            op.activate();
        }
    }
}
