use rayon::prelude::*;
use tokio::task;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

async fn find_primes_in_range(start: u32, end: u32) -> Vec<u32> {
    (start..end)
        .into_par_iter() // Paralel iteratör
        .filter(|&n| is_prime(n)) // Asal sayıları filtrele
        .collect() // Sonuçları topla
}

// Her aralık için bir asenkron görev oluşturun
async fn run() {
    let ranges = vec![
        (0, 10_000),
        (10_000, 20_000),
        (20_000, 30_000),
        (30_000, 40_000),
    ];

    // Her bir aralık için görevleri oluşturun
    let tasks: Vec<_> = ranges
        .into_iter()
        .map(|(start, end)| {
            task::spawn(async move {
                let primes = find_primes_in_range(start, end).await;
                println!(
                    "{} ile {} arasındaki asal sayılar: {:?}",
                    start, end, primes
                );
            })
        })
        .collect();

    // Tüm görevlerin tamamlanmasını bekleyin
    // Tokio join makrosunu kullanarak
    tokio::join!(tasks[0], tasks[1], tasks[2], tasks[3]);
}

#[tokio::main]
async fn main() {
    run().await; // run fonksiyonunu çağırın
}
