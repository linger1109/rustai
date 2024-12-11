use std::ops;
use std::ops::{Index, IndexMut};
// Definiton of a 'Vector' structure with various useful operations for training data
#[derive(Debug, Clone)]
pub struct Vector {
    dimensions: usize, 
    values: Vec<f64>
}

impl Vector {
    // Constructor to create a Vector with specified dimensions, with a default starting value 
    pub fn new_from_dims(dimensions: usize, default_val: f64) -> Self {
        let mut currentVals: Vec<f64> = Vec::new();
        for _i in 0..dimensions {
            currentVals.push(default_val);
        }
        
        Self {
            dimensions: dimensions,
            values: currentVals
        }
    }
    // Constructor to create a Vector from an inputted Vec<f64>. Copies inputted vector to values in self
    pub fn new_from_vec(fromVec: Vec<f64>) -> Self {
        Self {
            dimensions: fromVec.len(),
            values: fromVec
        }
    }
    // Returns Vector dimension as a usize
    pub fn get_dimension(&self) -> usize {
        return self.dimensions;
    }
    // Returns a clone of self.values as a Vector<f64>
    pub fn as_vec(&self) -> Vec<f64> {
        return self.values.clone(); // check if clone is necessary
    }
}
// Addition operation for Vector
impl ops::Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        assert_eq!(self.dimensions, rhs.dimensions);
        let mut v: Vec<f64> = Vec::new();
        for i in 0..self.dimensions {
            v.push(self[i] + rhs[i]);
        }
        return Vector::new_from_vec(v);
    }
}
// Subtraction operation for Vector
impl ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Vector {
        assert_eq!(self.dimensions, rhs.dimensions);
        let mut v: Vec<f64> = Vec::new();
        for i in 0..self.dimensions {
            v.push(self[i] - rhs[i]);
        }
        return Vector::new_from_vec(v);
    }
}
// Dot Product implementation for Vector
impl ops::Mul for Vector {
    type Output = f64;
    fn mul(self, rhs: Vector) -> f64 {
        assert_eq!(self.dimensions, rhs.dimensions);
        let mut result: f64 = 0.0;
        for i in 0..self.values.len() {
            result += self[i] * rhs[i];
        }
        return result;
    }
}
// Indexing operation for vector. Allows for immutable access
impl ops::Index<usize> for Vector {
    type Output = f64;
    fn index (&self, i: usize) -> &f64 {
        &self.values[i]
    }
}
// Indexing operation for vector. Allows for mutable access
impl ops::IndexMut<usize> for Vector {
    fn index_mut (&mut self, i: usize) -> &mut f64 {
        &mut self.values[i]
    }
}
// Returns the distance between two vectors (Euclidean)
pub fn vector_distance(v1: &Vector, v2: &Vector) -> f64 {
    assert_eq!(v1.dimensions, v2.dimensions);
    let mut s: f64 = 0.0;
    for i in 0..v1.dimensions {
        s += (v1[i] - v2[i]) * (v1[i] - v2[i]);
    }

    s.sqrt()
}
// Defines a Matrix structure. Useful for multidimensional data
#[derive(Debug, Clone)]
pub struct Matrix {
    width: usize,
    height: usize,
    values: Vec<Vector>
}

impl Matrix {
    // Constructor to create a [height x width] matrix with a default value
    pub fn new_from_dims(height: usize, width: usize, default_val: f64) -> Self {
        // initialise values, defaults to zero V
        let mut v: Vec<Vector> = Vec::new();
        for _i in 0..height {
            v.push(Vector::new_from_dims(width, default_val));
        }
        Self {width: width, height: height, values: v}
    }
    // Constructor to create a matrix from a mulitidimensional vec
    pub fn new_from_vec(v: Vec<Vec<f64>>) -> Self {
        // initialises Matrix from Vec<Vec>.
        // consumes the Vec<Vec> given.
        
        let mut vals: Vec<Vector> = Vec::new();

        let w = v[0].len();
        for i in v.iter() {
            if i.len() != w {
                // panic if jagged
                panic!();
            }

            // otherwise, continue
            vals.push(Vector::new_from_vec(i.to_vec()));
        }

        let h = v.len();

        Self {width: w, height: h, values: vals}

    }
    
    // Returns matrix dimensions
    pub fn get_dimensions(&self) -> (usize, usize) { (self.height, self.width) }
    // Returns matrix width
    pub fn get_width(&self) -> usize { self.width }
    // Returns matrix height
    pub fn get_height(&self) -> usize { self.height }
    // Computes the transpose of a matrix
    pub fn T(&self) -> Matrix {
        let mut res: Matrix = Matrix::new_from_dims(self.width, self.height, 0.0);
        for i in 0..self.height {
            for j in 0..self.width {
                res[j][i] = self[i][j];
            }
        }
        res
    }
}
// Index operator for immutable access
impl Index<usize> for Matrix {
    type Output = Vector;
    fn index(&self, index: usize) -> &Vector {
        &self.values[index]
    }
}
// Index operator for mutable access
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Vector {
        &mut self.values[index]
    }
}

// Addition operator for matrix
impl ops::Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);

        let mut v: Vec<Vec<f64>> = Vec::new();
        for _i in 0..self.height {
            v.push(vec![0.0; self.width]);
        }

        for i in 0..self.height {
            for j in 0.. self.width {
                v[i][j] = self[i][j] + rhs[i][j];
            }
        }

        Matrix::new_from_vec(v)

    }
}
// Subtraction operator for matrix
impl ops::Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);

        let mut v: Vec<Vec<f64>> = Vec::new();
        for _i in 0..self.height {
            v.push(vec![0.0; self.width]);
        }
        for i in 0..self.height {
            for j in 0..self.width {
                v[i][j] = self[i][j] - rhs[i][j];
            }
        }

        Matrix::new_from_vec(v)

    }
}
// Operator for matrix multiplication
impl ops::Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.width, rhs.height);
        //let mut v = vec![Vector::new_from_dims(self.height, 0); rhs.width];
        let mut v: Vec<Vec<f64>> = Vec::new();
        for _i in 0..self.width {
            v.push(vec![0.0; self.height]);
        }
        let transposed_rhs = rhs.T();
        for i in 0..self.height {
            for j in 0..transposed_rhs.height {
                let mut tmp: f64 = 0.0;
                for _k in 0..self.width {
                    tmp += self.values[i][j] * transposed_rhs.values[i][j];
                }
                v[i][j] = tmp;
            }
        }

        Matrix::new_from_vec(v)
    }
}


// Definition of a DataPoint structure for storing input/output pairs. Useful for training data
#[derive(Debug, Clone)]
pub struct DataPoint {
    input_dimensions: usize,
    output_dimensions: usize,
    input: Vector,
    output: Option<Vector>,
}
impl DataPoint {
    // Creates DataPoint with specified input_dimensions and output_dimensions.
    pub fn new_from_dims(input_dimensions: usize, output_dimensions: usize) -> Self {
        Self {
            input_dimensions,
            output_dimensions,
            input: Vector::new_from_dims(input_dimensions, 0.0),
            output: None
        }
    }
    // Creates DataPoint from a Vector for input and an Option<Vector> for output
    pub fn new_from_vec(input: Vector, output: Option<Vector>) -> Self {
        let input_dimensions: usize = input.get_dimension();
        let output_dimensions: usize = match &output {
            Some(vec) => vec.get_dimension(),
            None => 0,
        };
        Self {
            input_dimensions,
            output_dimensions,
            input,
            output
        }
    }
    // Returns a DataPoint's input as a Vector 
    pub fn get_input(&self) -> Vector {
        self.input.clone()
    }
    // Returns a DataPoint's output as Option<Vector>. Will need to unwrap to access
    pub fn get_output(&self) -> Option<Vector> {
        self.output.clone() 
    }
    // Returns dimensions as a tuple
    // 0th index is input dimension 1st index is output dimension
    pub fn get_dimension(&self) -> (usize, usize) {
        (self.input_dimensions, self.output_dimensions)
    }
}
