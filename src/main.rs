fn main(){
    fizzbuzz(3);
}

fn fizzbuzz(n: u32) -> String{
    let r;
    if n % 15 == 0{
        r = "FizzBuzz".to_string();
    }
    else if n % 3 == 0{
        r = "Fizz".to_string();
    }
    else if n % 5 == 0{
        r = "Buzz".to_string();
    }
    else {
        r = n.to_string();
    }
    println!("{r}");
    r
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fizzbuzz_multiple_of_three() {
        assert_eq!(fizzbuzz(3), "Fizz");
    }

    #[test]
    fn test_fizzbuzz_multiple_of_five() {
        assert_eq!(fizzbuzz(5), "Buzz");
    }

    #[test]
    fn test_fizzbuzz_multiple_of_three_and_five() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn test_fizzbuzz_return_2() {
        assert_eq!(fizzbuzz(2), "2");
    }
}