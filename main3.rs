mod util;

fn main() {
    // start by creating a builtin Vec<f64>
    let mut builtin = Vec::new();
    builtin.push(3.0);
    builtin.push(3.2);
    builtin.push(0.0);
    builtin.push(-12.2);

    println!("====== Vector ======");

    // now we will create two of our Vector objects
    let mut our1 = util::Vector::new_from_vec(builtin);
    let mut our2 = util::Vector::new_from_dims(4, 100.0);

    // print both of our Vectors
    println!("our1: {:?}", our1);
    println!("our2: {:?}", our2);

    // showing off the different operators we overloaded for our Vector type
    println!("our1 + our2: {:?}", our1.clone() + our2.clone());
    println!("our1 * our2: {:?}", our1.clone() * our2.clone());
    our1[1] = -5.0;
    println!("our1[1]: {:?}", our1[1]);
    println!("our1: {:?}", our1);

    println!("=====================");

    



    
    println!("\n\n\n");

    
    



    println!("====== Data Point ======");

    // making a DataPoint object
    let mut dataPoint = util::DataPoint::new_from_dims(4, 10);
    println!("dataPoint: {:?}", dataPoint);

    // creating two Vectors to show the alternative means of making a DataPoint object
    let v1 = util::Vector::new_from_vec(vec![1.0, 2.0, 3.0]);
    let v2 = util::Vector::new_from_vec(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let mut dataPoint2 = util::DataPoint::new_from_vec(v1, Some(v2));
    println!("dataPoint2: {:?}", dataPoint2);

    println!("=====================");
    
    
    

    println!("\n\n\n");

    
    
    
    println!("====== Matrices ======");

    // setting up the creation of a first matrix
    let mut test_mat1 = vec![Vec::new(); 3];
    test_mat1[0].push(1.0);
    test_mat1[0].push(2.0);
    test_mat1[1].push(3.0);
    test_mat1[1].push(4.0);
    test_mat1[2].push(5.0);
    test_mat1[2].push(6.0);

    // setting up the creation of a second matrix
    let mut test_mat2 = vec![Vec::new(); 2];
    test_mat2[0].push(1.0);
    test_mat2[0].push(2.0);
    test_mat2[0].push(3.0);
    test_mat2[1].push(4.0);
    test_mat2[1].push(5.0);
    test_mat2[1].push(6.0);

    // making three Matrix objects, one of which uses a different constructors
    let mut mat1 = util::Matrix::new_from_dims(3,2, 1.0);
    let mut mat2 = util::Matrix::new_from_vec(test_mat1);
    let mut mat3 = util::Matrix::new_from_vec(test_mat2);

    // Printing the three matrices
    println!("mat1: {:?}", mat1);
    println!("mat2: {:?}", mat2);
    println!("mat3: {:?}", mat3);

    // showing off some of the operators we have overloaded
    println!("mat1 + mat2: {:?}", mat1.clone() + mat2.clone());
    println!("mat1 * mat3: {:?}", mat1.clone() * mat3.clone());


    println!("=====================");   
}
