use crate::util::{Vector, Matrix, DataPoint};


pub struct LinearRegression {
   input_dimensions: usize,
   output_dimensions: usize,
   weights: Vector
}
impl LinearRegression {
   pub fn new(input_dimensions: usize, output_dimensions: usize) -> Self {
       LinearRegression {
           input_dimensions,
           output_dimensions,
           weights: Vector::new_from_dims(input_dimensions + 1, 0.0)
       }
   }
   pub fn new(points: &DataPoint) -> Self {
    Self {
        points.input_dimensions,
        points.output_dimensions,
        weights: Vector::new_from_dims(points.input_dimensions + 1, 0.0)
    }
   }
   pub fn fit(&mut self, training_data: &[DataPoint]) {// training data is a slice of datapoints
       // step configs we can change these depending on whatever works best. Also might want to put these in the input parameter
       let learning_rate = 0.0001;
       let iterations = 100000;
       // filtering out invalid/none values
       let mut valid_data = Vec::new();
       for data_point in training_data {
           if let Some(y) == &data_point.output {
               if y.get_dimension() == self.output_dimensions {
                   valid_data.push(data_point);
               }
           }
       }
      
       let n = valid_data.len();
       if n == 0 {
           return; 
       }


       let mut x_matrix = Matrix::new_from_dims(n, self.input_dimensions + 1, 0.0);
       let mut y_vec = Vector::new_from_dims(n, 0.0);


       for i in 0..n {
           x_matrix[i][0] = 1.0; // intercept
           let input_vec = valid_data[i].get_input();
           for j in 0..self.input_dimensions {
               x_matrix[i][j+1] = input_vec[j];
           }
           y_vec[i] = valid_data[i].output.as_ref().unwrap()[0];
       }


       // computing predictions
       for _ in 0..iterations {
           let mut predictions = Vector::new_from_dims(n, 0.0);
           for i in 0..n {
               let mut prediction = 0.0;
               for j in 0..(self.input_dimensions + 1) {
                   prediction += x_matrix[i][j] * self.weights[j];
               }
               predictions[i] = prediction;
           }
           // computing errors (predictions - correct y value)
           let mut errors = Vector::new_from_dims(n, 0.0);
           for i in 0..n {
               errors[i] = predictions[i] - y_vec[i];
           }
           // gradient = (1/N)*Σ(errors[i]*X[i][j]
           let mut gradient = Vector::new_from_dims(self.input_dimensions + 1, 0.0);
           for i in 0..(self.input_dimensions + 1) {
               let mut sum = 0.0;
               for j in 0..n {
                   sum += errors[j] * x_matrix[j][i];
               }
               gradient[i] = (1.0 / (n as f64) * sum);
           }
           // update the weights
           for i in 0..(self.input_dimensions + 1) {
               self.weights[i] -= learning_rate * gradient[i];
           }
       }
   }
   pub fn eval(&self, points: &[DataPoint]) -> f64 { // Finding the Mean Squared Error from the given points and the plotted Linear Regression Function
        let mut error:f64 = 0.0;

        for i in 0..self.input_dimensions {
            let mut prediction = 0.0;
            for j in 0..self.input_dimensions + 1 {
                prediction += points[i].input[j] * self.weights[j];
            }
            prediction += self.weights[self.input_dimensions - 1]; // Constant Term
            let mut diff = prediction - points[i].output[0];
            error += diff.abs() * diff.abs();
        }

        error
   }
   pub fn get_coefficients(&self) -> Vec<f64> {
       self.weights.as_vec()[1..].to_vec()
   }
   pub fn get_intercept(&self) -> f64 {
       self.weights[0]
   }
   pub fn get_dimensions(&self) -> (f64, f64) {
        (self.input_dimensions, self.output_dimensions)
   }
}
