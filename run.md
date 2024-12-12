# How to run
**1. Downloading the project**
* If `git` is installed on your system, run `git clone https://github.com/linger1109/skrusty-learn.git` on your Terminal app.
* Otherwise, hit 'Code⇢Download ZIP' from the [project main page](https://github.com/linger1109/skrusty-learn). Make sure you are grabbing the code from the main branch. Unzip the archive, and open Terminal from there.

**2. Code using the library**
Open `main.rs`. Import the library and code as you want. Code documentation is given below. Refer to `main1.rs`, `main2.rs`, `main3.rs` for example code files.

**3. Execute**
Type `cargo init` to initialize. Then type `cargo run` to execute. Note that `main.rs` is empty, so doing this will not do anything at first. However, the library does work and you can use all of the custom objects / machine learning models / functions that we have implemented. For demonstrations, you can copy and paste the entirety of the contents of any one of the three `main1.rs`, `main2.rs`, or `main3.rs` files into the `main.rs` file, and then do `cargo run` to see the output of that demo.

# Library Documentation
Note that this documentation is not comprehensive! We have some more functionality that is not covered here, however, this covers many of the basics for interfacing with and using our library.

Include `mod util` at the beginning of any file to have access to all of the utility objects necessary to interface with our machine learning models.
**util module**
* `Vector` struct: our custom struct for mathematical vectors, fixed number of dimensions and uses f64
 * constructor: `new_from_dims(dimensions: usize, default_val: f64)`. Makes a new vector with `dimensions` dimenions and the value `default_val` for evey component
 * constructor: `new_from_vec(fromVec: Vec<f64>)`. Makes a new vector with the components for every dimension given by `fromVec`
 * overloaded operators: `+` (vector addition), `-` (vector subtraction), `*` (vector dot product), `Index<usize>` (get component), `IndexMut<usize>` (get component mutably).
* `vector_distance(v1: &Vector, v2: &Vector) -> f64`: returns the distance between the tips of the two vectors when their tails are placed at the same point, equivalent to the magnitude of `v1-v2`. Note that this function takes `v1` and `v2` by reference.
* `Matrix` struct: our custom struct for mathematical matrices.
    * constructor: `new_from_dims(height: usize, width: usize, default_val: f64)`. Makes a new matrix of height `height`, width `width`, where `default_val` is the value in every matrix entry
    * constructor: `new_from_vec(Vec<Vec<f64>>)`. Makes a new matrix from the 2D `Vec` of `f64`.
    * member function `T(&self) -> Matrix`: returns the transposed matrix.
    * overloaded operators: `+` (matrix elementwise addition), `-` (matrix elementwise subtraction), `Index<usize>` (returns the passed in row of the matrix as a `Vector` object), `IndexMut<usize>` (same as `Index<usize>`, but is mutable).
* `DataPoint` struct: our custom structure to hold an input `Vector`, with an optional output (type `Option<Vector>`). This is useful for model training.
    * constructor: `new_from_dims(input_dimensions: usize, output_dimensions: usize) -> Self`: Makes a new DataPoint with specified `input_dimension` and `output_dimension`, with a default input Vector of size `input_dimensions` with default value of 0.0 and a default output of `None`
    *  constructor: `new_from_vec(input: Vector, output: Option<Vector>) -> Self`: Creates a Datapoint setting the `self.input` as the input Vector
    
Include `mod unsupervised` to have access to the unsupervised algorithm.
**unsupervised module**
* `KMeansClustering` struct: struct for K Means Clustering Algorithm.
    * constructor: `new(dimensions: usize, cluster_count: usize)`. Consturcts a new `struct` that will accept data points with `dimensions` dimensions and wil cluster data into `cluster_count` clusters.
    * `fit(data: Vec<util::Vector>)`: Fits the model using `data`. `data` is a Vec containing data points, where each data point is `util::Vector`. Each `util::Vector` must be of `self.dimensions` dimensions.
    * `eval(data: util::Vector) -> usize`: Evaluate the data point `data` according to the trained model. `data` is `self.dimensions`-dimension data point. Returns the id of the cluster that `data` is placed into.
    * `eval_batch(data: Vec<util::Vector>) -> Vec<usize>`: Evaluate multiple data points supplied by `Vec<util::Vector> data`. Returns `Vec<usize>`, parallel to `data`, wherc each element contains the id of the cluster that `data` is placed into.e
 
Include `mod supervised` to have access to the supervised algorithm.
 **supervised module**
* `LinearRegression` struct: struct for Linear Regression Algorithm
    * constructor option 1: `new_from_dims(input_dimensions: usize, output_dimensions: usize)` which will construct a new `struct` that will accept two `usize` variables to indicate input and output dimensions 
    * constructor option 2: `new_from_points(points: &DataPoint)` which will construct a new `struct` by accepting a sample `DataPoint`.
    * `fit(training_data: &[DataPoint])`: This function takes a slice of `DataPoint` by reference to read each point and fit a linear regression model
    * `eval(points: &[DataPoint])`: This function takes a slice of `DataPoint` by reference to evaluate the linear regression model through the given set of `DataPoint`. It outputs the calculated Mean Squared Error.
    * `get_coefficients()`: Returns the coefficients of the fitted model
    * `get_intercept()`: Returns the intercepts of the fitted model
    * `get_dimensions()`: Returns the dimensions of the input and output `DataPoint`
