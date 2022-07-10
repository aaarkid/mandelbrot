use num;
use std::str::FromStr;



fn escape_time(c: Complex<f64>, limit: u8) -> Option<u8> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i: u8 in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}