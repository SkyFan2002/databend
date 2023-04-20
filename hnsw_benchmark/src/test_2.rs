use std::collections::BinaryHeap;
use std::io::Write;

use faiss::index;
use faiss::index::io::write_index;
use faiss::index_factory;
use faiss::Index;
use faiss::MetricType;
use instant_distance::Builder;
use instant_distance::Point;
use instant_distance::Search;
use ndarray::ArrayView;
use prettytable::row;
use prettytable::Cell;
use prettytable::Row;
use prettytable::Table;

#[test]
fn test() {
    let mut table = Table::new();
    table.add_row(row![
        "向量个数",
        "维度",
        "K",
        "nprobe",
        "M",
        "索引构建时间",
        "IVF_PQ索引序列化后大小",
        "原始数据大小",
        "IVF_PQ搜索用时",
        "暴力搜索用时",
        "召回率"
    ]);
    table.add_row(bench(10000, 10, 50, 1, 10));
    table.add_row(bench(10000, 10, 50, 2, 10));
    table.add_row(bench(10000, 10, 50, 4, 10));
    table.add_row(bench(10000, 10, 50, 8, 10));

    // table.add_row(bench(100000, 10, 50, 1, 10));
    // table.add_row(bench(100000, 10, 50, 2, 10));
    // table.add_row(bench(100000, 10, 50, 4, 10));
    // table.add_row(bench(100000, 10, 50, 8, 10));
    table.printstd();
}

fn bench(num_points: usize, dim: usize, k: usize, nprobe: usize, m: usize) -> Row {
    assert!(dim % m == 0);

    let mut row = Row::empty();
    row.add_cell(Cell::new(&num_points.to_string()));
    row.add_cell(Cell::new(&dim.to_string()));
    row.add_cell(Cell::new(&k.to_string()));
    row.add_cell(Cell::new(&nprobe.to_string()));
    row.add_cell(Cell::new(&m.to_string()));

    let points = generate_points(num_points, dim);

    let start = std::time::Instant::now();
    let desp = format!("IVF{},PQ{}", nprobe, m);
    let mut index = index_factory(dim as u32, desp, MetricType::L2).unwrap();
    index.train(&points).unwrap();
    index.add(&points).unwrap();
    let elapsed = start.elapsed();
    row.add_cell(Cell::new(&format!("{:?}", elapsed)));
    let file = format!("{}{}{}{}{}.index", num_points, dim, k, nprobe, m);
    write_index(&index, &file).unwrap();
    let metadata = std::fs::metadata(&file).unwrap();
    let bytes = metadata.len();
    row.add_cell(Cell::new(&format!("{:?}KB", bytes as f64 / 1024.0)));
    row.add_cell(Cell::new(&format!("{:?}KB", num_points * dim * 4 / 1024)));
    let mut query = Vec::new();
    for _ in 0..dim {
        query.push(rand::random::<f32>());
    }
    let start = std::time::Instant::now();
    let result = index.search(&query, k).unwrap();
    let elapsed = start.elapsed();
    row.add_cell(Cell::new(&format!("{:?}", elapsed)));
    let indices = result
        .labels
        .iter()
        .map(|i| i.get().unwrap() as usize)
        .collect::<Vec<_>>();

    let points = points
        .chunks(dim)
        .map(|p| Point_(p.to_vec()))
        .collect::<Vec<_>>();
    let faiss_result = indices
        .iter()
        .map(|i| (points[*i].clone()))
        .collect::<Vec<_>>();
    let query = Point_(query);
    let start = std::time::Instant::now();
    let mut res = BinaryHeap::new();
    let mut buf = vec![0.0; num_points];
    let start = std::time::Instant::now();
    brute(&points, &mut buf, &query, &mut res, k);
    let res = res.into_sorted_vec();
    let elapsed = start.elapsed();
    row.add_cell(Cell::new(&format!("{:?}", elapsed)));

    let mut recall = 0;
    for i in 0..k {
        let faiss = &faiss_result[i].clone();
        for res in &res {
            if faiss == res.point {
                recall += 1;
                break;
            }
        }
    }
    row.add_cell(Cell::new(&(recall as f64 / k as f64).to_string()));
    row
}

fn generate_points(n: usize, d: usize) -> Vec<f32> {
    let mut points = Vec::new();
    for _ in 0..n * d {
        points.push(rand::random::<f32>() % 1024.0);
    }
    points
}

#[derive(Clone, Debug)]
struct Point_(Vec<f32>);

impl PartialEq for Point_ {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl instant_distance::Point for Point_ {
    fn distance(&self, other: &Self) -> f32 {
        //l2
        let mut dist = 0.0;
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            dist += (a - b) * (a - b);
        }
        dist
    }
}

fn brute<'a>(
    points: &'a [Point_],
    buf: &mut [f32],
    target: &Point_,
    res: &mut BinaryHeap<OrdPoint<'a>>,
    k: usize,
) {
    for (point, buf) in points.iter().zip(buf.iter_mut()) {
        *buf = point.distance(&target);
    }
    for i in 0..k {
        res.push(OrdPoint {
            point: &points[i],
            dist: buf[i],
        });
    }
    for i in k..points.len() {
        if buf[i] < res.peek().unwrap().dist {
            res.pop();
            res.push(OrdPoint {
                point: &points[i],
                dist: buf[i],
            });
        }
    }
}

struct OrdPoint<'a> {
    point: &'a Point_,
    dist: f32,
}

impl<'a> Ord for OrdPoint<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.partial_cmp(&other.dist).unwrap()
    }
}

impl<'a> PartialOrd for OrdPoint<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl<'a> PartialEq for OrdPoint<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<'a> Eq for OrdPoint<'a> {}
