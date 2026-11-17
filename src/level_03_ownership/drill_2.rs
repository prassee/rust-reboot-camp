#[cfg(test)]
mod tests {
    #[test]
    fn test_snippet1() {
        fn shout(text: String) -> String {
            text.to_uppercase()
        }

        // rewrite the method to take reference to avoid cloning - Give ownership back
        fn shout_with_ref(text: &str) -> String {
            text.to_uppercase()
        }
        let greeting = String::from("hello");
        let a = shout_with_ref(&greeting);
        let b = shout_with_ref(&greeting);
        println!("{:?} {:?}", a, b);

        let greeting = String::from("hello");
        let greeting_clone = greeting.clone();
        let a = shout(greeting);
        // E0382 - borrow of moved value
        // let b = shout(greeting);

        // the below one compiles using greeting_clone
        let b = shout(greeting_clone.clone());
        println!("{:?} {:?}", a, b);

        // or the same idea as above
        // but different approach
        let greeting = String::from("hello");
        let a = shout(greeting.clone());
        let b = shout(greeting);
        println!("{:?} {:?}", a, b);
    }

    #[test]
    fn test_snippet2() {
        fn consume(v: Vec<i32>) -> usize {
            v.len()
        }

        fn consume_ref(v: &[i32]) -> usize {
            v.len()
        }

        let numbers = vec![1, 2, 3];
        // let count = consume(numbers);
        // E0382 - borrow of moved value
        // Fix - restructured the code by passing the numbers first & then move numbers to consume.
        print!("{:?} items in ", numbers);
        println!("{:?}", consume(numbers));

        // pass the  reference to consume_ref
        let numbers = vec![1, 2, 3];
        let count = consume_ref(&numbers);
        println!("{count} items in {:?}", numbers);

        // fix pass the cloned numbers to consume &
        // pass the numebrs to println
        let numbers = vec![1, 2, 3];
        let count = consume(numbers.clone());
        println!("{count} items in {:?}", numbers);
    }

    #[test]
    fn test_snippet3() {
        // fix - Config to derive
        #[derive(Debug, Clone)]
        struct Config {
            name: String,
            retries: u32,
        }

        fn print_name(name: String) {
            println!("{name}");
        }

        fn print_name_ref(name: &str) {
            println!("{name}");
        }

        let config = Config {
            name: String::from("prod"),
            retries: 3,
        };
        print_name(config.name);
        // E0382 - borrow of moved value config.name
        // println!("{} {}", config.name, config.retries);

        // print_name(config.name);
        let config = Config {
            name: String::from("prod"),
            retries: 3,
        };
        // restructured code to print the config and procced to move to print_name
        println!("{} {}", config.name, config.retries);
        print_name(config.name);

        // Fix - pass cloned version of config
        let config = Config {
            name: String::from("prod"),
            retries: 3,
        };
        // pass only reference
        print_name_ref(config.name.as_str());
        println!("{} {}", config.name, config.retries);
    }

    #[test]
    fn test_snippet4() {
        // let label = String::from("item");
        // let mut all = Vec::new();
        // for i in 0..3 {
        // E0382 - borrow of moved value
        // all.push(format!("{}-{i}", label));
        // let owned = label;
        // println!("{:?}", owned);
        // }

        let label = String::from("item");
        let mut all = Vec::new();
        for i in 0..3 {
            all.push(format!("{}-{i}", label));
            let owned: &str = &label;
            println!("{:?}", owned);
        }

        // restructrure code - move label inside loop, println only takes reference and unblocks to send label to push in to vector.
        for i in 0..3 {
            let label = String::from("item");
            println!("{:?}", label);
            all.push(format!("{}-{i}", label));
        }

        println!("{:?}", all);
    }
}
