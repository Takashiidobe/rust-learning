use criterion::{Criterion, black_box, criterion_group, criterion_main};
use sqlx::{SqlitePool, query};
use tokio::runtime::Runtime;

async fn run_query(pool: &SqlitePool) {
    // Insert
    query("INSERT INTO songs (artist, title) VALUES (?, ?)")
        .bind("Kenshi Yonezu")
        .bind("Lemon")
        .execute(pool)
        .await
        .unwrap();

    // Fetch it back
    let _row: (String,) = sqlx::query_as("SELECT artist FROM songs WHERE title = ?")
        .bind("Lemon")
        .fetch_one(pool)
        .await
        .unwrap();
}

fn sqlite_benchmark(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    // Setup DB and migrations once outside the measurement loop
    let pool = rt.block_on(async {
        let pool = SqlitePool::connect("sqlite::memory:?cache=shared")
            .await
            .expect("failed to connect to sqlite in-memory");

        // Run migrations from ./migrations relative to Cargo.toml
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("failed to run migrations");

        pool
    });

    c.bench_function("sqlite in-memory insert+select", |b| {
        // Clone pool so each iteration can use it
        let pool = pool.clone();

        b.to_async(&rt).iter(|| async {
            run_query(&pool).await;
            black_box(())
        })
    });
}

criterion_group!(benches, sqlite_benchmark);
criterion_main!(benches);
