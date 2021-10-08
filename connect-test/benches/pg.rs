use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use tokio_postgres::{Client, NoTls};

async fn connect(conn_str: &str) -> anyhow::Result<Client> {
    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

fn connect_test(rt: &Runtime, conn_str: &str) {
    rt.block_on(async {
        connect(conn_str).await.unwrap();
    })
}

fn _query_test(rt: &Runtime, client: &Client, query: &str) {
    rt.block_on(async {
        client.query(query, &[]).await.unwrap();
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let pg13 = "postgresql://postgres:prisma@127.0.0.1:5435/postgres";
    let pg14 = "postgresql://postgres:prisma@127.0.0.1:5437/postgres";

    c.bench_function("pg13 connect", |b| b.iter(|| connect_test(&rt, black_box(pg13))));
    c.bench_function("pg14 connect", |b| b.iter(|| connect_test(&rt, black_box(pg14))));

    /*
    c.bench_function("pg13 select", |b| {
        let client = rt.block_on(async { connect(pg13).await.unwrap() });
        b.iter(|| query_test(&rt, &client, black_box("SELECT 1")));
    });

    c.bench_function("pg14 select", |b| {
        let client = rt.block_on(async { connect(pg14).await.unwrap() });
        b.iter(|| query_test(&rt, &client, black_box("SELECT 1")));
    });
    */
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
