use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

fn main() {
    let start = std::time::Instant::now();

    broadcast_message("HI");
    broadcast_message("IS ANYONE HERE?");

    let duration = start.elapsed();
    println!("Completed in {:?}", duration);
}

fn broadcast_message(input: &str) {
    println!("Sending message: {:#?}", input);

    // Rc ve RefCell ile counter'ı tanımlıyoruz
    let counter = Rc::new(RefCell::new(0));

    for _ in 0..1000 {
        let counter = Rc::clone(&counter);

        // Counter'ı artırıyoruz
        {
            let mut counter_guard = counter.borrow_mut(); // RefCell ile mutable referans alıyoruz
            *counter_guard += 1; // Counter'ı artırıyoruz
            let msg = counter_guard.to_string(); // Artırılmış değeri mesaj olarak alıyoruz
            send_message(msg); // Mesajı gönderiyoruz
        }

        // Simüle edilmiş bir gecikme
        std::thread::sleep(Duration::from_millis(300));
    }

    let final_count = *counter.borrow(); // Son değeri alıyoruz
    println!("Final count: {}", final_count);
}

fn send_message(input: String) {
    read_message(input);
}

fn read_message(input: String) {
    println!("User {:#?} received the message", input);
}
