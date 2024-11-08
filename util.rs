use std::ops;

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

    fn new_from_vec(fromVec: Vec<i32>) -> Self {
        Self {
            dimensions: fromVec.len(),
            values: fromVec
        }
    }

    fn get_dimension() -> usize {
        return dimensions;
    }

    fn as_vec() -> Vec<f64> {
        return values.clone(); // check if clone is necessary
    }
}

impl ops::Add for Vector {
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
    fn mul(self, rhs: Vector) -> f64 {
        assert_eq!(self.dimensions, rhs.dimensions);
        let mut result: f64 = 0;
        for i in 0..self.values.size() {
            result += self[i] * rhs[i];
        }
        return result;
    }
}

impl ops::Index<usize> for Vector {
    fn index (self, i: usize) -> f64 {
        self.values[i];
    }
}

impls ops::IndexMut<usize> for Vector -> &mut f64{
    fn index (self, i: usize) -> f64 {
        self.values[i];
    }
}

pub class Matrix {
    width: usize,
    height: usize,
    values: Vec<Vector>
}

impl Matrix {
    pub fn new(height: f64, width: f64) -> Self {
        // initialise values, defaults to zero V
        let mut v: Vec<Vector> = vec![Vector::new_from_dims(width, 0); height];
        Self {width: width, height: height, values: v}
    }

    pub fn new_from_vec(v: vec<Vector>) -> Self {
        // todo: check that the matrix is rectangular        
        Self {
            width: v[]
        }
    }

    pub fn getDimensions() -> (usize, usize) { (height, width) }
        
    pub fn getWidth() -> usize { width }
    
    pub fn getHeight() -> usize { height }

    pub fn T() -> Matrix {
        let mut res: Matrix = Matrix::new(width, height);
        for i in 0..height {
            for j in 0..width {
                res[j][i] = self[i][j];
            }
        }
        res
    }
}

impl Index<usize> for Matrix {
    type Output = Vector;
    fn index(&self, index: usize) -> &Self::Output {
        self.values[index]
    }
}

impl IndexMut<usize> for Matrix {
    type Output = Vector;
    fn index(&self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}


impl ops::Add for Matrix {
    fn add(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);

        let mut v: Vec<Vector> = vec![Vector::new_from_dims(self.width, 0); self.height];

        for i in 0..self.height {
            for j in 0.. self.width {
                v[i][j] = self[i][j] + rhs[i][j];
            }
        }

        Matrix::new_from_vec(v)

    }
}

impl ops::Sub for Matrix {
    fn subtract(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.width, rhs.width);
        assert_eq!(self.height, rhs.height);

        let mut v: Vec<Vector> = vec![Vector::new_from_dims(self.width, 0); self.height];

        for i in 0..self.height {
            for j in 0..self.width {
                v[i][j] = self[i][j] - rhs[i][j];
            }
        }

        Matrix::new_from_vec(v)

    }
}

impl ops::Mult for Matrix {
    pub fn multiply(self, rhs: Matrix) {
        assert_eq!(self.width, self.height); 

    }
}




pub struct DataPoint {
    input_dimensions: usize,
    output_dimensions: usize,
    input: Vector,
    output: Option<Vector>,
}
impl DataPoint {
    pub fn new(input_dimensions: usize, output_dimensions: usize) -> Self {
        Self {
            input_dimensions,
            output_dimensions,
            input: Vector::new(input_dimensions),
            output: None
        }
    }
    pub fn with_vec(Vector& input, Option<Vector>& output) -> Self {
        let input_dimensions: usize = input.len();
        let output_dimensions: usize = output.unwrap().len();
        Self {
            input_dimensions,
            output_dimensions,
            input,
            output
        }
    }
}