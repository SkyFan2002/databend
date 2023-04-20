#[test]
fn test() {
    use faiss::index_factory;
    use faiss::Index;
    use faiss::MetricType;

    let mydata = vec![1.0, 2.0, 3.0];
    let query = vec![4.0, 1.0, 3.0];
    let mut index = index_factory(3, "Flat", MetricType::InnerProduct).unwrap();
    index.add(&mydata).unwrap();

    let result = index.search(&query, 1).unwrap();
    println!("{:?}", result.distances);
}
