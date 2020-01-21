use std::fmt::Write;

pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    let pling = n % 3 == 0;
    let plang = n % 5 == 0;
    let plong = n % 7 == 0;

    if pling {
        write!(s, "Pling").unwrap();
    }
    if plang {
        write!(s, "Plang").unwrap();
    }
    if plong {
        write!(s, "Plong").unwrap();
    }

    if !(pling || plang || plong) {
        write!(s, "{}", n).unwrap();
    }

    s
}
