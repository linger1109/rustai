use std::ops;
use std::ops::{Index, IndexMut};

#[derive(Default)]
pub struct Vector {
    dimensions: usize,
    values: Vec<f64>
}

impl Vector {
    fn new_from_dims(dimensions: usize, default_val: f64) -> Self {
        let mut currentVals: Vec<f64> = Vec::new();
        for i in 0..dimensions {
            currentVals.push(default_val);
        }
        
        Self {
            dimensions: dimensions,
            values: currentVals
        }
    }

    fn new_from_vec(fromVec: Vec<f64>) -> Self {
        Self {
            dimensions: fromVec.len(),
            values: fromVec
        }
    }

    fn get_dimension(&self) -> usize {
        return self.dimensions;
    }

    fn as_vec(&self) -> Vec<f64> {
        return self.values.clone(); // check if clone is necessary
    }
}

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

impl ops::Index<usize> for Vector {
    type Output = f64;
    fn index (&self, i: usize) -> &f64 {
        &self.values[i]
    }
}

impl ops::IndexMut<usize> for Vector {
    fn index_mut (&mut self, i: usize) -> &mut f64 {
        &mut self.values[i]
    }
}

pub struct Matrix {
    width: usize,
    height: usize,
    values: Vec<Vector>
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Self {
        // initialise values, defaults to zero V
        let mut v: Vec<Vector> = vec![Vector::new_from_dims(0, width); height]; // ERROR "the trait `Clone` is not implemented for `Vector`"
        Self {width: width, height: height, values: v}
    }

    pub fn new_from_vec(v: Vec<Vector>) -> Self {
        // todo: check that the matrix is rectangular        
        todo!();
        // }
    }

    pub fn getDimensions(&self) -> (usize, usize) { (self.height, self.width) }
        
    pub fn getWidth(&self) -> usize { self.width }
    
    pub fn getHeight(&self) -> usize { self.height }

    pub fn T(&self) -> Matrix {
        let mut res: Matrix = Matrix::new(self.width, self.height);
        for i in 0..self.height {
            for j in 0..self.width {
                res[j][i] = self[i][j];
            }
        }
        res
    }
}

impl Index<usize> for Matrix {
    type Output = Vector;
    fn index(&self, index: usize) -> &Vector {
        &self.values[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Vector {
        &mut self.values[index]
    }
}


impl ops::Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);

        let mut v: Vec<Vector> = vec![Vector::new_from_dims(self.width, 0); self.height]; // ERROR "the trait `Clone` is not implemented for `Vector`"

        for i in 0..self.height {
            for j in 0.. self.width {
                v[i][j] = self[i][j] + rhs[i][j];
            }
        }

        Matrix::new_from_vec(v)

    }
}

impl ops::Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);

        let mut v: Vec<Vector> = vec![Vector::new_from_dims(self.width, 0.0); self.height]; // ERROR "the trait `Clone` is not implemented for `Vector`"

        for i in 0..self.height {
            for j in 0..self.width {
                v[i][j] = self[i][j] - rhs[i][j];
            }
        }

        Matrix::new_from_vec(v)

    }
}

impl ops::Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        // todo
        todo!();
    }
}




pub struct DataPoint {
    input_dimensions: usize,
    output_dimensions: usize,
    input: Vector,
    output: Option<Vector>,
}
impl DataPoint {
    pub fn new_from_dims(input_dimensions: usize, output_dimensions: usize) -> Self {
        Self {
            input_dimensions,
            output_dimensions,
            input: Vector::new_from_dims(input_dimensions, 0.0),
            output: None
        }
    }
    pub fn with_vec(input: Vector, output: Option<Vector>) -> Self {
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
}
