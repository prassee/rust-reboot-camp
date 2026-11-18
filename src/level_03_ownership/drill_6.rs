#[cfg(test)]
mod tests {
    #[test]
    fn test_copy_move_clone() {
        // Copy since i32 implement Copy trait
        let a = 5;
        let b = a;
        println!("{a} {b}");

        // String does not implment Copy hence moved
        let a = String::from("x");
        let b = a;
        // E0382 - move of borrowed value a is dropped only b is usable
        println!("{b}");

        // tuples does not implment Copy as said in the brief
        let a = (1, String::from("x"));
        // E0382 - move of borrowed value a is dropped only b is usable
        let b = a;
        println!("{:?}", b);
    }
}
