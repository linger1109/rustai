mod util;
mod unsupervised;
mod supervised;

fn main() {
    let mut unsupervised_object = unsupervised::KMeansClustering::new(1, 2);

    let mut test_data_pre: Vec<f64> = vec![-7.6, -8.2, -7.0, -8.4, 7.6, 8.2, 7.0, 8.4];
    
    let mut test_data: Vec<util::Vector> = Vec::new();
    for i in 0..8 {
        test_data.push(util::Vector::new_from_vec(vec![test_data_pre[i]]));
    }

    unsupervised_object.fit(test_data);

    print!("{}", unsupervised_object.eval(util::Vector::new_from_dims(1, -7.5)));
    print!("{}", unsupervised_object.eval(util::Vector::new_from_dims(1, -8.5)));

    print!("{}", unsupervised_object.eval(util::Vector::new_from_dims(1, 7.5)));
    print!("{}", unsupervised_object.eval(util::Vector::new_from_dims(1, 8.0)));
    print!("{}", unsupervised_object.eval(util::Vector::new_from_dims(1, 7.0)));
    print!("{}", unsupervised_object.eval(util::Vector::new_from_dims(1, 9.0)));

    print!("{}", unsupervised_object.eval(util::Vector::new_from_dims(1, -1.0)));
}
