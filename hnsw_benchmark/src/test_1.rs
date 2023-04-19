use std::collections::BinaryHeap;
use std::io::Write;

use instant_distance::Builder;
use instant_distance::Point;
use instant_distance::Search;
use ndarray::ArrayView;
use prettytable::row;
use prettytable::Cell;
use prettytable::Row;
use prettytable::Table;
use serde::Serialize;

#[test]
fn test() {
    let mut table = Table::new();
    table.add_row(row![
        "向量个数",
        "维度",
        "K",
        "EF",
        "EF_CONSTRUCTION",
        "索引构建时间",
        "HNSW索引序列化(serde_json)后大小",
        "原始数据大小",
        "HNSW搜索用时",
        "暴力搜索用时",
        "召回率"
    ]);
    table.add_row(bench(1000, 10, 50, 50, 10));
    table.add_row(bench(10_000, 10, 50, 50, 10));
    table.add_row(bench(100_000, 10, 50, 50, 10));
    table.add_row(bench(1000_000, 10, 50, 50, 10));
    table.add_row(bench(10_000, 20, 50, 50, 10));
    table.add_row(bench(10_000, 50, 50, 50, 10));
    table.add_row(bench(10_000, 100, 50, 50, 10));
    table.add_row(bench(1000, 10, 10, 10, 10));
    table.add_row(bench(10_000, 10, 10, 10, 10));
    table.add_row(bench(100_000, 10, 10, 10, 10));
    table.add_row(bench(1000_000, 10, 10, 10, 10));
    table.add_row(bench(10_000, 20, 10, 10, 10));
    table.add_row(bench(10_000, 50, 10, 10, 10));
    table.add_row(bench(10_000, 100, 10, 10, 10));

    table.add_row(bench(1000, 3, 50, 50, 10));
    table.add_row(bench(10_000, 3, 50, 50, 10));
    table.add_row(bench(100_000, 3, 50, 50, 10));
    table.add_row(bench(1000_000, 3, 50, 50, 10));
    table.add_row(bench(10_000, 3, 50, 50, 10));
    table.add_row(bench(10_000, 3, 50, 50, 10));
    table.add_row(bench(10_000, 3, 50, 50, 10));
    table.add_row(bench(1000, 3, 10, 10, 10));
    table.add_row(bench(10_000, 3, 10, 10, 10));
    table.add_row(bench(100_000, 3, 10, 10, 10));
    table.add_row(bench(1000_000, 3, 10, 10, 10));
    table.add_row(bench(10_000, 3, 10, 10, 10));
    table.add_row(bench(10_000, 3, 10, 10, 10));
    table.add_row(bench(10_000, 3, 10, 10, 10));
    table.printstd();
}

fn bench(num_points: usize, dim: usize, k: usize, ef: usize, ef_construction: usize) -> Row {
    let mut row = Row::empty();
    row.add_cell(Cell::new(&num_points.to_string()));
    row.add_cell(Cell::new(&dim.to_string()));
    row.add_cell(Cell::new(&k.to_string()));
    row.add_cell(Cell::new(&ef.to_string()));
    row.add_cell(Cell::new(&ef_construction.to_string()));
    let points = generate_points(num_points, dim);
    let points_clone = points.clone();

    let start = std::time::Instant::now();
    let (hnsw, _) = Builder::default()
        .ef_construction(ef_construction)
        .ef_search(ef)
        .build_hnsw(points);
    let elapsed = start.elapsed();
    row.add_cell(Cell::new(&format!("{:?}", elapsed)));

    let serialized = serde_json::to_string(&hnsw).unwrap();
    let mut file = std::fs::File::create("hnsw.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();

    row.add_cell(Cell::new(&format!(
        "{:?}KB",
        serialized.len() as f64 / 1024.0
    )));
    row.add_cell(Cell::new(&format!("{:?}KB", num_points * dim * 4 / 1024)));

    let mut search = Search::default();
    let mut target = Vec::new();
    for _ in 0..dim {
        target.push(rand::random::<f32>());
    }
    let target = Point_(target);

    let start = std::time::Instant::now();
    let mut hnsw_res = Vec::new();
    let mut closest_points = hnsw.search(&target, &mut search);
    for _ in 0..k {
        hnsw_res.push(closest_points.next().unwrap().point);
    }
    let elapsed = start.elapsed();
    row.add_cell(Cell::new(&format!("{:?}", elapsed)));

    let mut res = BinaryHeap::new();
    let mut buf = vec![0.0; num_points];
    let start = std::time::Instant::now();
    brute(&points_clone, &mut buf, &target, &mut res, k);
    let res = res.into_sorted_vec();
    let elapsed = start.elapsed();
    row.add_cell(Cell::new(&format!("{:?}", elapsed)));

    let mut recall = 0;
    for i in 0..k {
        let hnsw = hnsw_res[i];
        for res in &res {
            if hnsw == res.point {
                recall += 1;
                break;
            }
        }
    }
    row.add_cell(Cell::new(&(recall as f64 / k as f64).to_string()));
    row
}

#[derive(Clone, Debug, Serialize)]
struct Point_(Vec<f32>);

impl PartialEq for Point_ {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl instant_distance::Point for Point_ {
    fn distance(&self, other: &Self) -> f32 {
        cosine_distance(&self.0, &other.0)
    }
}

pub fn cosine_distance(from: &[f32], to: &[f32]) -> f32 {
    let a = ArrayView::from(from);
    let b = ArrayView::from(to);
    let aa_sum = (&a * &a).sum();
    let bb_sum = (&b * &b).sum();
    1.0 - (&a * &b).sum() / ((aa_sum).sqrt() * (bb_sum).sqrt())
}

fn generate_points(n: usize, d: usize) -> Vec<Point_> {
    let mut points = Vec::new();
    for _ in 0..n {
        let mut point = Vec::new();
        for _ in 0..d {
            point.push(rand::random::<f32>());
        }
        points.push(Point_(point));
    }
    points
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
