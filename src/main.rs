use std::io;

fn main() {
    println!("Temperatue Converter\n\n\n");

    loop {
        // println!("Would you like Celsius or Fahrenheit?");
        // println!("Please enter C or F\n");

        let mut choice: char = temp_choice();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        
        println!("Enter \"END\" to halt the program\n");

        if choice == "C" {

            println!("Enter the temperature you want to convert\n");

            let mut tempc = String::new();
            let mut user_continue = String::new();

            io::stdin()
                .read_line(&mut tempc)
                .expect("Failed to read line");

            let tempc: f64 = match tempc.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            println!("You inputted: {}\n", tempc);

            println!("Celsius temp is: {}\n", (calc_tempc(tempc)).to_string());
            
            println!("Would you like to do another? Y or N\n");

            io::stdin()
                .read_line(&mut user_continue)
                .expect("Failed to read line");

            if user_continue.trim() == "Y" {
                continue;
            } else if user_continue.trim() == "N" {
                break;
            }

        } else if choice == "F" {

            println!("Enter the temperature you want to convert\n");
            println!("Enter \"END\" to halt the program");

            let mut tempf = String::new();
            let mut user_continue = String::new();

            io::stdin()
                .read_line(&mut tempf)
                .expect("Failed to read line");

            let tempf: f64 = match tempf.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            println!("You inputted: {}\n", tempf);

            println!("Fahrenheit temp is: {}\n", (calc_tempf(tempf)).to_string());

            println!("Would you like to do another? Y or N\n");

            io::stdin()
                .read_line(&mut user_continue)
                .expect("Failed to read line");

            if user_continue.trim() == "Y" {
                continue;
            } else if user_continue.trim() == "N" {
                break;
            }

        } else if choice == "END" {
            break;
        }
    }
}

fn temp_choice() -> &str {
    println!("Enter the temperature you'd like to convert: F or C?");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    loop {
        if choice == "F" {
            "F"
        } else if choice == "C" {
            "C"
        } else if choice == "END" {
            break;
        } else {
            println!("Not a valid choice.");
            continue
        }
    }
}
    



fn calc_tempc(temp: f64) -> f64 {
   temp / 5.0 * 9.0 + 32.0
}

fn calc_tempf(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}
