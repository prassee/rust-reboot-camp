#[cfg(test)]
mod tests {
    #[test]
    #[allow(unused_mut)]
    fn test_a() {
        let mut s: String = String::from("a");
        let rl = &s;
        let r2 = &s;
        // compiles both point to s
        println!("{rl} {r2}");
    }

    #[test]
    fn test_b() {
        let mut s = String::from("a");
        let m1 = &mut s;
        //E0499 this is invalid since multiple mutable borrow more than once
        // let m2 = &mut s;
        m1.push('x');
        // m2.push('y');
    }

    #[test]
    #[allow(unused_mut)]
    fn test_c() {
        let mut s = String::from("a");
        let r = &s;
        // E0502 - mutable borrow while a immutable exists
        // let m = &mut s;
        // m.push('x');
        println!("{r}");
    }

    #[test]
    fn test_d() {
        let mut s = String::from("a");
        // compiles the immuatble borrow r is dropped with the println
        let r = &s;
        println!("{r}");
        // unblcoked to borrow as mutable - hence no E0502 error
        let m = &mut s;
        m.push('x');
    }

    #[test]
    fn test_e() {
        let mut s = String::from("a");
        let m = &mut s;
        // same E0502 - using s as immutable
        // moving the println stmt after m.push would fix this issue
        // println!("{s}");
        m.push('b');
    }

    #[test]
    #[allow(unused_mut)]
    fn test_f() {
        // let mut s = String::from("a");
        let mut v = [1, 2, 3]; //vec![1, 2, 3];
        // same E0502 - v.push requires a mutable ref but first is using a immutable ref
        // hence both cannot exist at the same time as we seen in 3.3 where a push is called inside the loop.
        // the same applies here as well but without the loop
        // moving the println stmt after v.push would fix this issue
        let first = &v[0];
        // v.push(4);
        println!("{first}");
    }

    #[test]
    #[allow(unused_mut)]
    fn test_g() {
        // let mut s = String::from("a");
        let mut n = 5;
        let r = &n;
        // E0506 - n is borrowed hence cannot assign
        // re-order next 2 statements would fix the issue
        // n = 6;
        println!("{r}");
    }

    #[test]
    fn test_h() {
        // let mut s = String::from("a");
        // compiles - no immutables borrows here
        let mut v: Vec<usize> = vec![1, 2, 3];
        v.push(v.len());
    }
}
