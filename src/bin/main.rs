use idiomatic::money;

fn main() {
    let money = "4 $".parse::<money::Money>().unwrap();
    println!("{money:#?}");
    println!("{:?},", money.amount)
}
