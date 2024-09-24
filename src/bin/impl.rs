struct Temperature {
    degrees_f: f64,
}

impl Temperature {
    // this method takes in the Temperature as parameter, more like a this keyword in javascript
    fn show_temp(&self) {
        println!("{:?} is the temperature in degree F", self.degrees_f)
    }

    // this is me using the impl to create a new temperature
    fn freezing() -> Self {
        Self { degrees_f: 45.9 }
    }
}

fn main() {
    let hot = Temperature { degrees_f: 99.8 };
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp();
}
