use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;
use tokio_postgres::{config::SslMode, Client, Config, NoTls};

async fn connect(port: u16) -> anyhow::Result<Client> {
    let mut config = Config::new();

    config.user("postgres");
    config.password("prisma");
    config.host("127.0.0.1");
    config.port(port);
    config.dbname("postgres");
    config.ssl_mode(SslMode::Disable);

    let (client, connection) = config.connect(NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

fn full_connect_test(rt: &Runtime, port: u16) {
    rt.block_on(async {
        connect(port).await.unwrap();
    })
}

fn simple_query_test(rt: &Runtime, client: &Client) {
    rt.block_on(async {
        client.query("SELECT 1", &[]).await.unwrap();
    })
}

fn criterion_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    let pg13 = 5435;
    let pg14 = 5437;

    c.bench_function("pg14 full connect", |b| {
        b.iter(|| full_connect_test(&rt, black_box(pg14)))
    });

    c.bench_function("pg13 full connect", |b| {
        b.iter(|| full_connect_test(&rt, black_box(pg13)))
    });

    c.bench_function("pg14 select 1", |b| {
        let client = rt.block_on(async { connect(pg14).await.unwrap() });
        b.iter(|| simple_query_test(&rt, &client));
    });

    c.bench_function("pg13 select 1", |b| {
        let client = rt.block_on(async { connect(pg13).await.unwrap() });
        b.iter(|| simple_query_test(&rt, &client));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
