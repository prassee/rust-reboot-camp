pub fn take_string(s: String) -> usize {
    s.len()
}
pub fn borrow_str(s: &str) -> usize {
    s.len()
}
pub fn take_number(n: i32) -> i32 {
    n + 1
}
pub fn take_and_return(s: String) -> String {
    s
}
pub fn take_tuple(t: (i32, String)) -> i32 {
    t.0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_take_string() {
        // allocated in heap
        let s = "abcd".to_string();
        // move s to calling fn & dropped by the caller i.e it returns the len
        println!("{}", take_string(s));
        // compiler error - error[E0382]: borrow of moved value: `s`
        // println!("{}", s);

        // where as passing the cloned s works to take_string and to println
        let s = "abcd".to_string();
        take_string(s.clone());
        println!("{}", s);

        let t = (1, String::from("x"));
        // partially  move values from tuple
        take_string(t.1);
        println!("{}", t.0);
        // errors out borrow of partially moved value t - error[E0382]: borrow of partially moved value: `t`
        // println!("{:?}", t);

        let s = "abcd".to_string();
        // r just hold the reference & lives as long as s , once moved using r should throw compiler error
        let r = &s; // using _ instead of a variable name
        // r is usable since s still a valid ref and not yet moved in to take_string()
        // this is moved next to take_string, its a compiler error as seen before.
        println!("{:?}", r);
        // s moved here & hence r cannot hence using r further would return an error.
        take_string(s);

        // error error[E0505]: cannot move out of `s` because it is borrowed
        //  println!("{:?}", r);
    }

    #[test]
    fn test_borrow_string() {
        // allocated in heap
        let s = "abcd".to_string();
        // calling fn borrows s ref  & not dropped by the caller
        println!("{}", borrow_str(&s));
        // s still usable - expected no compiler error
        println!("{}", s);
    }

    #[test]
    fn test_take_number() {
        // allocated in stack
        let n = 5;
        take_number(n);
        println!("{}", n);
    }

    #[test]
    fn test_take_tuple() {
        // tuple allocated in stack whereas the string lives in the heap
        let x = (1, "ping".to_string());
        // move x to take_tuple & dropped by the caller i.e it returns the first element of tuple
        take_tuple(x);
        // compiler error - error[E0382]: borrow of moved value: `x`
        // println!("{:?}", x.0);
    }

    #[test]
    fn test_take_and_return() {
        let s = String::from("hi");
        // take_and_return returns a owned string hence it can be used further
        let s = take_and_return(s);
        println!("{}", s)
    }
}
