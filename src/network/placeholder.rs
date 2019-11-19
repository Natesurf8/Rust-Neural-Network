pub struct Placeholder {
    data: Vec<f64>,
    shape: Vec<usize>,
    name: String,
}
impl Placeholder {
    pub fn new(name: &str, shape: Vec<usize>) -> Placeholder {
        let mut len: usize = 1;
        for i in &shape {
            len *= i;
        }

        Placeholder {
            name: String::from(name),
            data: vec![0.0; len],
            shape: shape,
        }
    }
    pub fn get_shape(&self) -> &[usize] {
        return &self.shape;
    }
    pub fn get_data(&self) -> &[f64] {
        return &self.data;
    }
    pub fn get_data_mut(&mut self) -> &mut [f64] {
        return &mut self.data;
    }
    pub fn get_data_clone(&self) -> Vec<f64> {
        return self.data.clone();
    }

    pub fn copy_data(&mut self, data: &[f64]) {
        assert!(data.len() == self.data.len(), format!("data length's were mismatched, attempted: {}, currently: {}", data.len(), self.data.len()));

        for i in 0..self.data.len() {
            self.data[i] = data[i];
        }
    }
}
