use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
use tokio_postgres::NoTls;

static CONN_STR: Lazy<String> = Lazy::new(|| std::env::var("CONN_STR").unwrap());

async fn connect_test() -> anyhow::Result<()> {
    let (client, connection) = tokio_postgres::connect(&*CONN_STR, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client.query_one("SELECT $1", &[&"hello"]).await?;

    Ok(())
}

fn sync_connect_test(rt: &Runtime) -> anyhow::Result<()> {
    rt.block_on(async { connect_test().await })
}

fn main() -> anyhow::Result<()> {
    let rt = Runtime::new().unwrap();

    sync_connect_test(&rt)?;

    Ok(())
}
