pub mod util;
pub mod unsupervised;
pub mod supervised;
use crate::supervised::LinearRegression;
use crate::util::{Vector, DataPoint};

fn main() {
    // create all of the training_data
    let mut training_data: Vec<DataPoint> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let x1 = i as f64;
            let x2 = j as f64;
            let y = 5.0 + 2.0 * x1 + 3.0 * x2; // we find the output as this function of the two inputs.
            let input = Vector::new_from_vec(vec![x1, x2]); // Two input dimensions
            let output = Some(Vector::new_from_vec(vec![y])); 
            training_data.push(DataPoint::new_from_vec(input, output)); // add this value to the training_data
        }
    }

    let mut model = LinearRegression::new_from_dims(2, 1); // Two input dimensions, one output dimension
    model.fit(&training_data); // fit the model with our data

    // print the results of training
    let intercept = model.get_intercept();
    let coefficients: Vec<f64> = model.get_coefficients();
    println!("Learned intercept: {}", intercept);
    println!("Learned coefficients: {:?}", coefficients);

    // see how well our model approximates the data
    let mse = model.eval(&training_data);
    println!("Mean Squared Error on training data: {}", mse);
}   
