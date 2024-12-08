use crate::util;

pub struct KMeansClustering {
    dimensions: usize,
    cluster_count: usize,
    cluster_centroids: Vec<util::Vector>, // each element in the Vec is one of the centroids, and each centroid is described by a Vector (since it exists in n-dimensional space).
}

impl KMeansClustering {
    const THRESHOLD_CHANGE: f64 = 0.001; // min change in centroids' position required to stop training

    pub fn new(dimensions: usize, cluster_count: usize) -> Self {
        Self {
            dimensions: dimensions,
            cluster_count: cluster_count,
            cluster_centroids: Vec::new() // defined empty for now, will be properly initialized in fit
        }
    }

    pub fn fit(&mut self, data: Vec<util::Vector>) {
        assert!(self.cluster_count <= data.len());

        // clear (if any) previous training
        self.cluster_centroids.clear();

        // not sure how to initialise centroids; just taking first n values for now. Doesn't work when some datas are equal.
        for i in 0..self.cluster_count {
            self.cluster_centroids.push(util::Vector::new_from_vec(data[i].as_vec()));
        }
        
        // scratch spaces; defining here to avoid continuously allot'ing memory
        let mut allocated_clusters: Vec<usize> = vec![0; data.len()];
        let mut cluster_datasum: Vec<util::Vector> = vec![util::Vector::new_from_dims(self.dimensions, 0.0); self.cluster_count]; // scratch space for computiung sum of centroids
        let mut cluster_counter: Vec<usize> = vec![0; self.cluster_count]; // how many datapoints are in the ith cluster?

        loop {
            // no idea why rust doesn't have do whiles, but using this instead

            let mut max_ds: f64 = 0.0; // maximum change of distance of centroids from prev iteration

            // wipe scratch space data (if it needs to be wiped)
            for i in 0..self.cluster_count {
                cluster_datasum[i] = util::Vector::new_from_dims(self.dimensions, 0.0);
                cluster_counter[i] = 0;
            }

            // determine closest centroids for each
            for i in 0..data.len() {
                let mut closest: usize = 0;
                let mut best_dist: f64 = f64::MAX;
                for j in 0..self.cluster_count {
                    let d: f64 = util::vector_distance(&data[i], &self.cluster_centroids[j]);
                    if d < best_dist {
                        closest = j; // #j is the closest centroid
                        best_dist = d;
                    }
                }
                allocated_clusters[i] = closest;
            }

            // for each centroid, find the average, i.e. new centroid.
            for i in 0..data.len() {
                let cluster_id: usize = allocated_clusters[i];
                cluster_counter[cluster_id] += 1;
                cluster_datasum[cluster_id] = cluster_datasum[cluster_id].clone() + data[i].clone();
            }

            // divide to find the average
            for i in 0..self.cluster_count {
                // better if we can divide Vector by a scalar, don't think that's a thing now
                for j in 0..self.dimensions {
                    cluster_datasum[i][j] /= cluster_counter[i] as f64;
                }
            }

            // copy values over, and compute the difference on the way
            for i in 0..self.cluster_count {
                let ds = util::vector_distance(&self.cluster_centroids[i], &cluster_datasum[i]);
                if ds > max_ds {
                    max_ds = ds;
                }
                self.cluster_centroids[i] = cluster_datasum[i].clone();
            }

            //print!("{}", max_ds);
            if max_ds <= Self::THRESHOLD_CHANGE {
                break;
            }
        }
    }

    pub fn eval(&self, data: util::Vector) -> usize {
        assert!(self.cluster_centroids.len() > 0);

        let mut best: usize = 0;
        let mut best_dist: f64 = util::vector_distance(&data, &self.cluster_centroids[0]);

        for i in 1..self.cluster_count {
            let this_dist: f64 = util::vector_distance(&data, &self.cluster_centroids[i]);
            if this_dist < best_dist {
                best_dist = this_dist;
                best = i;
            }
        }

        return best;
    }

    pub fn eval_batch(&self, data: Vec<util::Vector>) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();

        for i in 0..data.len() {
            res.push(self.eval(data[i].clone()));
        }

        return res;
    }
}
