use std::io;

fn main() {
    println!("Temperatue Converter\n\n\n");

    loop {
        println!("Would you like Celsius or Fahrenheit?");
        println!("Please enter C or F\n");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        println!("Enter \"END\" to halt the program\n");

        if choice.trim() == "C" {

            println!("Enter the temperature you want to convert\n");

            let mut tempc = String::new();

            io::stdin()
                .read_line(&mut tempc)
                .expect("Failed to read line");

            let tempc: f64 = match tempc.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            println!("You inputted: {}\n", tempc);

            println!("Celsius temp is: {}\n", (calc_tempc(tempc)).to_string());

        } else if choice.trim() == "F" {

            println!("Enter the temperature you want to convert\n");
            println!("Enter \"END\" to halt the program");

            let mut tempf = String::new();

            io::stdin()
                .read_line(&mut tempf)
                .expect("Failed to read line");

            let tempf: f64 = match tempf.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            println!("You inputted: {}\n", tempf);

            println!("Fahrenheit temp is: {}\n", (calc_tempf(tempf)).to_string());

        } else if choice.trim() == "END" {
            break;
        }
    }
}

fn calc_tempc(temp: f64) -> f64 {
   temp / 5.0 * 9.0 + 32.0
}

fn calc_tempf(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}
