use databend_driver::new_connection;
use futures_util::StreamExt;

#[tokio::main]
async fn main() {
    let dsn = "your-dsn";
    let conn = new_connection(&dsn).unwrap();
    conn.exec("DROP TABLE IF EXISTS test").await.unwrap();
    conn.exec(
        "create table test (
        id bigint,
        id2 bigint,
        id3 bigint,
        id4 bigint,
        id5 bigint,
        id6 bigint,
        id7 bigint
        ) CLUSTER BY(id);",
    )
    .await
    .unwrap();
    for i in 1..=100 {
        conn.exec(&format!("insert into test select number,number *2,number * 3,number * 4,number *5 ,number * 6,number *7 from numbers({}) order by number DESC limit 1000;", i * 1000))
        .await
        .unwrap();
        println!("insert done {}", i);
    }
    let mut rows = conn.query_iter("SELECT sum(id) FROM test").await.unwrap();
    let (sum,): (i64,) = rows.next().await.unwrap().unwrap().try_into().unwrap();
    assert_eq!(sum, 100000 * 99999 / 2);
    println!("sum done");

    println!("insert done");

    let dsn_clone = dsn.clone();
    let delete_handle = tokio::spawn(async move {
        let conn = new_connection(&dsn_clone).unwrap();
        let mut success: i64 = 0;
        for i in 0..1000 {
            println!("delete start, {}", i);
            let result = conn
                .exec(&format!("DELETE FROM test WHERE id < {}", i * 10))
                .await;
            match result {
                Ok(_) => {
                    success = i;
                    println!("delete done, {}", i);
                }
                Err(e) => {
                    println!("delete failed {}", e);
                }
            }
        }
        success
    });
    let compact_handle = tokio::spawn(async move {
        let conn = new_connection(&dsn).unwrap();
        for i in 0..300 {
            println!("optimize start, {}", i);
            let _ = conn
                .exec("OPTIMIZE TABLE test COMPACT SEGMENT LIMIT 10")
                .await;
            println!("compact segment done, {}", i);
            let _ = conn.exec("OPTIMIZE TABLE test COMPACT LIMIT 10").await;
            println!("compact block done, {}", i);
        }
    });
    let deleted = delete_handle.await.unwrap() * 10;
    println!("deleted {}", deleted);
    compact_handle.await.unwrap();
    let mut rows = conn.query_iter("SELECT count(*) FROM test").await.unwrap();
    let (count,): (i64,) = rows.next().await.unwrap().unwrap().try_into().unwrap();
    assert_eq!(count, 100000 - deleted);
    println!("count done");
    let mut rows = conn
        .query_iter(
            "SELECT sum(id),sum(id2),sum(id3),sum(id4),sum(id5),sum(id6),sum(id7) FROM test",
        )
        .await
        .unwrap();
    let (id, id2, id3, id4, id5, id6, id7): (i64, i64, i64, i64, i64, i64, i64) =
        rows.next().await.unwrap().unwrap().try_into().unwrap();
    let sum: i64 = sum - (deleted - 1) * deleted / 2;
    assert_eq!(id, sum);
    assert_eq!(id2, sum * 2);
    assert_eq!(id3, sum * 3);
    assert_eq!(id4, sum * 4);
    assert_eq!(id5, sum * 5);
    assert_eq!(id6, sum * 6);
    assert_eq!(id7, sum * 7);
    println!("sum done");
    let mut rows = conn
        .query_iter("select * from system.metrics where metric like 'fuse%';")
        .await
        .unwrap();
    while let Some(row) = rows.next().await {
        let row = row.unwrap();
        println!("{:?}", row);
    }
}
