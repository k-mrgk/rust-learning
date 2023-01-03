use std::fmt;

#[derive(Debug)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    fn convert(&self) -> Temperature {
        match *self {
            Temperature::Celsius(ref u) => Temperature::Fahrenheit((u * 9.0 / 5.0) + 32.0),
            Temperature::Fahrenheit(ref u) => Temperature::Celsius((u - 32.0) * 5.0 / 9.0),
        }
    }
}
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Temperature::Celsius(ref u) => write!(f, "{}", u),
            Temperature::Fahrenheit(ref u) => write!(f, "{}", u),
        }
    }
}

fn main() {
    // In Celsius
    let c = Temperature::Celsius(100.0);
    let r = c.convert();

    println!("Celsius: {} to Fahrenheit: {}", c, r);

    // // In Fahrenheit
    let f = Temperature::Fahrenheit(0.0);
    let r = f.convert();

    println!("Fahrenheit: {} to Celsius: {}", f, r);
}
