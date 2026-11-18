#[cfg(test)]
mod tests {
    #[test]
    fn test_copy_move_clone() {
        // a
        // Copy since i32 implement Copy trait
        let a = 5;
        let b = a;
        println!("{a} {b}");

        // b
        // String does not implment Copy hence moved
        let a = String::from("x");
        let b = a;
        // E0382 - move of borrowed value a is dropped only b is usable
        println!("{b}");

        // c
        // tuples does not implment Copy as said in the brief
        let a = (1, String::from("x"));
        // E0382 - move of borrowed value a is dropped only b is usable
        let b = a;
        println!("{:?}", b);

        // d
        let a = [1, 2, 3];
        let b = a;
        println!("{:?}", a);

        // e
        let a = vec![1, 2, 3];
        let b = a;
        println!("{:?}", b);

        // f
        struct P {
            x: i32,
            y: i32,
        }

        let a = P { x: 1, y: 2 };
        let b = a;
        // println!("{:?}", b);
        //

        // g
        let a = Some(1);
        let b = a;
        println!("{:?}", a);
        println!("{:?}", b);

        // h
        let a = Some(String::from("x"));
        let b = a;
        // [TODO] fix this
        // println!("{:?}", a);
        println!("{:?}", b);

        // i
        let s = String::from("x");
        let a = &s;
        println!("{:?}", a);
        println!("{:?}", b);

        // j
        let mut s = String::from("x");
        let a: &mut String = &mut s;
        println!("{:?}", a);
        println!("{:?}", b);
    }
}
