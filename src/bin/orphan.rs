use idiomatic::money::{self, MoneyError};
use std::process::{ExitCode, Termination};

fn main() -> Result<(), MoneyError> {
    // Parse the money string into a Money instance
    let money = "4 tl".parse::<money::Money>()?;

    // Print the money instance using Pretty Debug format
    println!("{:#?}", money);

    // Print the amount of money
    println!("Amount: {:?}", money.get_amount());

    // Create a MoneyWrapper and call the report method
    MoneyWrapper(money).report();

    Ok(())
}

// Wrapper struct for Money to implement Termination
struct MoneyWrapper(money::Money);

// The orphan rule is not violated
// because we are implementing Termination for MoneyWrapper
impl Termination for MoneyWrapper {
    fn report(self) -> ExitCode {
        // Return a successful exit code
        ExitCode::SUCCESS
    }
}
