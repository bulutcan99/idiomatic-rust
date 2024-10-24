use std::sync::Arc;
use tokio::{
    sync::Mutex,
    task,
    time::{sleep, Duration},
};

// tokio good for IO-bound tasks -> IO operations, network operations, file operations
#[tokio::main]
async fn main() {
    let start = tokio::time::Instant::now();
    broadcast_message("IS ANYONE HERE?").await;
    let duration = start.elapsed();
    println!("Completed in {:?}", duration);
}

async fn broadcast_message(_input: &str) {
    println!("Broadcasting messages");

    let counter = Arc::new(Mutex::new(0));
    let mut tasks = vec![];

    for _ in 0..1000 {
        let counter = Arc::clone(&counter);

        let task = task::spawn(async move {
            let mut counter_guard = counter.lock().await;
            *counter_guard += 1; // Counter'ı artırıyoruz
            let msg = counter_guard.to_string(); // Artırılmış değeri mesaj olarak alıyoruz
            drop(counter_guard); // Kilidi serbest bırakıyoruz

            send_message(msg).await;
        });

        tasks.push(task);
    }

    // Bütün task'ların tamamlanmasını bekliyoruz
    for task in tasks {
        let _ = task.await;
    }

    let final_count = *counter.lock().await;
    println!("Final count: {}", final_count);
}

async fn send_message(input: String) {
    read_message(input).await;
    sleep(Duration::from_millis(300)).await;
}

async fn read_message(input: String) {
    println!("User {:#?} received the message", input);
}
