pub struct KMeansClustering {
    dimensions: usize,
    cluster_count: usize, // please rename this
    cluster_centroids: Vec<Vector> // each element in the Vec is one of the centroids, and each centroid is described by a Vector (since it exists in n-dimensional space).

    const THRESHOLD_CHANGE = 0.001; // min change in centroids' position required to stop training
}

impl KMeansClustering {
    pub fn new(dimensions: usize, cluster_count: usize) -> Self {
        Self {
            dimensions: dimensions,
            cluster_count: cluster_count
        }
    }

    pub fn fit(data: Vec<Vector>) {
        assert!(cluster_count <= data.len());

        // clear any previous training, if any
        cluster_centroids.clear();

        // not sure how to initialise centroids; just taking first n values for now. Doesn't work when some datas are equal.
        for i in 0..cluster_count {
            cluster_centroids.push(Vector.new_from_vec(data[i]));
        }

        let mut max_ds: f64 = 1000; // maximum change of distance of centroids from prev iteration
        
        // scratch spaces; defining here to avoid continuously allot'ing memory
        let mut allocated_clusters: Vec<usize> = vec![0; data.len()];
        let mut cluster_datasum = Vec<Vector> = vec![Vector::new_from_dims(dimensions, 0); cluster_count]; // scratch space for computiung sum of centroids
        let mut cluster_counter = Vec<usize> = vec![0; cluster_count]; // how many datapoints are in the ith cluster?

        loop {
            // no idea why rust doesn't have do whiles, but using this instead

            // wipe scratch space data (if it needs to be wiped)
            for i in 0..cluster_count {
                cluster_datasum[i] = Vector::new_from_dims(dimensions, 0);
                cluster_counter[i] = 0;
            }

            // determine closest centroids for each
            for i in 0..data.len() {
                let mut closest: usize = 0;
                let mut best_dist: f64 = f64::MAX;
                for j in 0..cluster_count {
                    let mut d: f64 = vector_distance(data[i], cluster_centroids[j]);
                    if d < best_dist {
                        closest = j; // #j is the cloest centroid
                        best_dist = d;
                    }
                }
                allocated_clusters[i] = cloest;
            }

            // for each centroid, find the average, i.e. new centroid.
            for i in 0..data.len() {
                let cluster_id: usize = allocated_clusters[i];
                cluster_counter[cluster_id] += 1;
                cluster_datasum[cluster_id] += data[i];
            }

            // divide to find the average
            for i in 0..cluster_count {
                // better if we can divide Vector by a scalar, don't think that's a thing now
                for j in 0..dimensions {
                    cluster_datasum[i][j] /= cluster_counter[i];
                }
            }

            // copy values over, and compute the difference on the way
            for i in 0..cluster_count {
                let mut ds = vector_distance(cluster_centroids[i], cluster_datasum[i]);
                max_ds = cmp::max(max_ds, ds);
                cluster_centroids = cluster_datasum[i];
            }

            if max_ds <= THRESHOLD_CHANGE {
                break;
            }
        }
    }

    pub fn eval(data: Vector) -> usize {
        assert!(cluster_centroids.len() > 0);

        let mut best: usize = 0;
        let mut best_distance: f64 = vector_distance(data, cluster_centroids[0]);

        for i in 1..cluster_count {
            let this_dist: f64 = vector_distance(data, cluster_centroids[i]);
            if (this_dist < best_dist) {
                best_dist = this_dist;
                best = i;
            }
        }

        return best;
    }

    pub fn eval_batch(data: Vec<Vector>) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();

        for i in 0..data.len() {
            res.push(eval(data[i]));
        }

        return res;
    }
}
