// What is Copying ?
// The Copy trait implemting types let the values get copied from one var to another.
// The copied ones get own address(thats my understanding correct me !!).
// 3 takeaways about Copying
// types implemnting Copy trait can be re-assigned , else thorws compiler error while copying
// String, Vec and &mut references never copy - hence moving them never compiles
// Struct & Option - copy if all the memebers do implment copy else never compiles
#[cfg(test)]
mod tests {
    #[test]
    fn test_a() {
        // Copy since i32 implement Copy trait
        let a = 5;
        let _b = a;
        println!("{a}");
    }

    #[test]
    fn test_b() {
        // String does not implment Copy hence moved
        let a = String::from("x");
        let _b = a;
        // E0382 - move of borrowed value a is dropped only b is usable
        // println!("{a}");
    }

    #[test]
    fn test_c() {
        // does not compiles - tuples onlu Copy if everything inside implements Copy
        let a = (1, String::from("x"));
        // E0382 - move of borrowed value a is dropped only b is usable
        let _b = a;
        // println!("{:?}", a);
    }

    #[test]
    fn test_d() {
        // compiles i32 type implement copy & hence compiles
        let a = [1, 2, 3];
        let _b = a;
        println!("{:?}", a);
        // println!("{:?}", b);
    }

    #[test]
    fn test_e() {
        // does not complie Vec never implement Copy
        let a = vec![1, 2, 3];
        let _b = a;
        // println!("{:?}", a);
    }

    #[test]
    fn test_f() {
        // #[derive(Debug)]
        struct P {
            _x: i32,
            _y: i32,
        }
        // does not Implement Debug
        // & with #[derive(Debug)] , does not compile - Struct does not implement copy
        let a = P { _x: 1, _y: 2 };
        let _b = a;
        // println!("{:?}", a);
    }

    #[test]
    fn test_g() {
        let a = Some(1);
        let b = a;
        println!("{:?}", a);
        println!("{:?}", b);
    }

    #[test]
    fn test_h() {
        let a = Some(String::from("x"));
        let _b = a;
        // does not compile same as Tuple since String does not implemnt Copy
        // println!("{:?}", a);
    }
    #[test]
    fn test_i() {
        // compiles - copies the reference of s in to a
        let s = String::from("x");
        let a = &s;
        let _b = a;
        println!("{:?}", a);
    }
    #[test]
    fn test_j() {
        // compiles - copies the reference of s in to a - same as i
        let mut s = String::from("x");
        let a: &mut String = &mut s;
        let _b = a;
        // E0382 - move of borrowed value a is dropped only b is usable
        // println!("{:?}", a);
    }
}
