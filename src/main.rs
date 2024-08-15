use std::io;

fn main() {
    
    let mut temperature_input = String::new();
    println!("Please enter a temperature:");
    io::stdin().read_line(&mut temperature_input).expect("Failed to recieve temperature");
    let temperature = temperature_input;

    let mut temperature_lower_boundary_input = String::new();
    println!("Please enter a lower bound:");
    io::stdin().read_line(&mut temperature_lower_boundary_input).expect("Failed to receive lower boundary");
    let temperature_lower_boundary = temperature_lower_boundary_input;

    let mut temperature_upper_boundary_input = String::new();
    println!("Please enter an upper bound:");
    io::stdin().read_line(&mut temperature_upper_boundary_input).expect("Failed to receive upper boundary");
    let temperature_upper_boundary = temperature_upper_boundary_input;

    if temperature < temperature_lower_boundary {
        println!("The temperature you entered is lower than your lower boundary. TOO COLD!");
    }
    else if temperature > temperature_upper_boundary {
        println!("The temperature you entered is higher than your upper boundary. TOO HOT!");
    }
    else {
        println!("You supplied a reasonable temperature, compared to the lower and upper bounds.");
    }
}
