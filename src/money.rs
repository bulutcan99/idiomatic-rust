pub fn take_money(input: &str) -> Result<i32, std::num::ParseIntError> {
    let splits: Vec<&str> = input.split_whitespace().collect();
    let amount = splits[0].parse::<i32>()?;
    Ok(amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = take_money("4 $");
        assert_eq!(result.unwrap(), 4);
    }
}
