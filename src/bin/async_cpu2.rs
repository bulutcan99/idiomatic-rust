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
        .into_par_iter() // Rayon ile paralel iteratör
        .filter(|&n| is_prime(n)) // Asal sayıları filtrele
        .collect() // Sonuçları topla
}

#[tokio::main]
async fn main() {
    let ranges = vec![
        (0, 10_000),
        (10_000, 20_000),
        (20_000, 30_000),
        (30_000, 40_000),
    ];

    let mut tasks = vec![];

    for (start, end) in ranges {
        let task = task::spawn(async move {
            let primes = find_primes_in_range(start, end).await;
            println!(
                "{} ile {} arasındaki asal sayılar: {:?}",
                start, end, primes
            );
        });
        tasks.push(task);
    }

    // Tüm görevlerin tamamlanmasını bekleyin
    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Görev hatası: {:?}", e);
        }
    }
}
