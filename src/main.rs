use std::io;

fn main() {
    println!("Temperatue Converter\n\n\n");

    loop {
        println!("Enter the temperature you want to convert\n");
        println!("Enter \"END\" to halt the program");

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        if temperature.trim()  == "END" {
            break;
        }

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You inputted: {}", temperature);

        println!("{}", (tempc(temperature)).to_string());
    }
}

fn tempc(temp: f64) -> f64 {
   temp / 5.0 * 9.0 + 32.0
}

fn tempf(temp: f64) -> {
    (temp - 32.0) * 5.0 / 9.0
}
