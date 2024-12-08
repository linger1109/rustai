pub struct KMeansClustering {
    dimensions: usize,
    cluster_centroids: Vec<Vector> // each element in the Vec is one of the centroids, and each centroid is described by a Vector (since it exists in n-dimensional space).
}