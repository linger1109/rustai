pub mod util;
pub mod unsupervised;
pub mod supervised;
use crate::supervised::LinearRegression;
use crate::util::{Vector, DataPoint};

fn main() {
    // SINGLE DIMENSIONS
    // y = 5.0 + 2x
    // let mut training_data: Vec<DataPoint> = Vec::new();
    // for i in 0..50 {
    //     let x = i as f64;
    //     let y = 5.0 + 2.0*x;
    //     let input = Vector::new_from_vec(vec![x]);
    //     let output = Some(Vector::new_from_vec(vec![y])); 
    //     training_data.push(DataPoint::new_from_vec(input, output));
    // }

    // let mut model = LinearRegression::new_from_dims(1, 1);
    // model.fit(&training_data);

    // let intercept = model.get_intercept();
    // let coefficients: Vec<f64> = model.get_coefficients();

    // println!("Learned intercept: {}", intercept);
    // println!("Learned coefficients: {:?}", coefficients);

    // let mse = model.eval(&training_data);
    // println!("Mean Squared Error on training data: {}", mse);
    // // Adding an outlier our out of the norm in the training data
    // training_data.push(DataPoint::new_from_vec(
    //     Vector::new_from_vec(vec![200.0]),
    //     Some(Vector::new_from_vec(vec![734.0])),
    // ));

    // let mse_with_noise = model.eval(&training_data);
    // println!("Mean Squared Error with noisy data: {}", mse_with_noise);

    // MULTIDIMENSIONS
    // y = 5.0 + 2x_1 + 3x_2


    let mut training_data: Vec<DataPoint> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let x1 = i as f64;
            let x2 = j as f64;
            let y = 5.0 + 2.0 * x1 + 3.0 * x2 + 0.1 * x1* x1;
            let input = Vector::new_from_vec(vec![x1, x2]); // Two input dimensions
            let output = Some(Vector::new_from_vec(vec![y])); 
            training_data.push(DataPoint::new_from_vec(input, output));
        }
    }

    let mut model = LinearRegression::new_from_dims(2, 1); // Two input dimensions, one output dimension
    model.fit(&training_data);

    let intercept = model.get_intercept();
    let coefficients: Vec<f64> = model.get_coefficients();

    println!("Learned intercept: {}", intercept);
    println!("Learned coefficients: {:?}", coefficients);

    let mse = model.eval(&training_data);
    println!("Mean Squared Error on training data: {}", mse);

    // Adding an outlier
    training_data.push(DataPoint::new_from_vec(
        Vector::new_from_vec(vec![100.0, 200.0]), // 
        Some(Vector::new_from_vec(vec![1500.0])),
    ));

    let mse_with_noise = model.eval(&training_data);
    println!("Mean Squared Error with noisy data: {}", mse_with_noise);

}   