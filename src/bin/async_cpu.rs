use log::Level;
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::task;

// Fibonacci hesaplama (rekürsif)
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib(n - 1) + fib(n - 2),
    }
}

// Asenkron uyuma fonksiyonu
async fn sleeper() {
    log::info!("Sleeping...");
    tokio::time::sleep(Duration::from_millis(300)).await;
    log::info!("Awake!");
}

// Asenkron dosya okuma fonksiyonu
async fn reader() {
    log::info!("Reading some big data...");

    let mut file = tokio::fs::File::open("one_billion_row.csv").await.unwrap();
    let mut contents = vec![];
    file.read_to_end(&mut contents).await.unwrap();

    log::info!("Read big {} bytes", contents.len());

    // CPU bloklayan işlemi başka bir thread'de çalıştır
    task::spawn_blocking(move || {
        log::info!("Computing fib(40)...");
        let result = fib(40);
        log::info!("Done computing fib(40): {}", result);
    })
    .await
    .unwrap();
}

// Asenkron görevlerin hepsini çalıştıran fonksiyon
async fn run() {
    tokio::join!(sleeper(), reader(), reader(), reader(), reader(), reader(),);
}

// Ana fonksiyon
#[tokio::main]
async fn main() {
    // Logger'ı başlat
    simple_logger::init_with_level(Level::Info).unwrap();

    // Başlangıç zamanını kaydet
    let start = std::time::Instant::now();

    // Tüm görevleri çalıştır
    run().await;

    // Tamamlanma zamanını ölç
    let end = std::time::Instant::now();
    println!("Took {:?} seconds", end - start);
}
