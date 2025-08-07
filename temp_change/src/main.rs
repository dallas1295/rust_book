use std::io;

fn main() {
    let mut active = true;

    while active {
        let mut opt = String::new();
        let mut temp = String::new();

        println!("Are you converting Celsius to Fahrenheit? (c or C)");
        println!("Or Fahrenheit to Celsius? (f or F)");

        io::stdin().read_line(&mut opt).expect("Error reading line");
        let opt = opt.trim();

        if !opt.eq_ignore_ascii_case("c") && !opt.eq_ignore_ascii_case("f") {
            println!("Please answer c or f.");
            continue;
        }

        println!("What's the temperature?");
        io::stdin()
            .read_line(&mut temp)
            .expect("Error reading line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature input.");
                continue;
            }
        };

        if opt.eq_ignore_ascii_case("c") {
            let converted = c_to_f(temp);
            println!("The conversion is {:.2} degrees Fahrenheit", converted);
        } else {
            let converted = f_to_c(temp);
            println!("The conversion is {:.2} degrees Celsius", converted);
        }

        println!("Would you like to convert again? (yes/no)");

        let mut again = String::new();
        io::stdin()
            .read_line(&mut again)
            .expect("Error reading line");
        let again = again.trim();

        if again.eq_ignore_ascii_case("no") {
            active = false;
        }
    }
}

fn c_to_f(temp: f64) -> f64 {
    (9.0 / 5.0) * temp + 32.0
}

fn f_to_c(temp: f64) -> f64 {
    (5.0 / 9.0) * (temp - 32.0)
}
