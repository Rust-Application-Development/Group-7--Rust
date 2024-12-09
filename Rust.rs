use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)] // Add Clone trait to make ParkingSpot cloneable
struct ParkingSpot {
    is_occupied: bool,
    customer_name: Option<String>,
    vehicle_number: Option<String>,
    booking_time: Option<u64>, // in seconds since UNIX_EPOCH
}

fn main() {
    let mut parking_spots = vec![ParkingSpot {
        is_occupied: false,
        customer_name: None,
        vehicle_number: None,
        booking_time: None,
    }; 30];

    loop {
        println!("\nOptions:");
        println!("1. Display parking spots");
        println!("2. Book a parking spot");
        println!("3. Release a parking spot");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => display_parking_spots(&parking_spots),
            "2" => {
                let spot = get_spot_number("Enter the spot number to book: ");
                book_parking_spot(&mut parking_spots, spot);
            }
            "3" => {
                let spot = get_spot_number("Enter the spot number to release: ");
                release_parking_spot(&mut parking_spots, spot);
            }
            "4" => {
                println!("Exiting the parking system.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

fn display_parking_spots(parking_spots: &[ParkingSpot]) {
    println!("Parking spots:");
    for (i, spot) in parking_spots.iter().enumerate() {
        let status = if spot.is_occupied {
            format!(
                "Occupied by {}, Vehicle No: {}",
                spot.customer_name.as_ref().unwrap(),
                spot.vehicle_number.as_ref().unwrap()
            )
        } else {
            "Empty".to_string()
        };
        println!("Spot {}: {}", i + 1, status);
    }
}

fn book_parking_spot(parking_spots: &mut [ParkingSpot], spot_number: usize) {
    if 1 <= spot_number && spot_number <= 30 {
        let spot = &mut parking_spots[spot_number - 1];
        if !spot.is_occupied {
            print!("Enter customer name: ");
            io::stdout().flush().unwrap();
            let mut customer_name = String::new();
            io::stdin().read_line(&mut customer_name).expect("Failed to read input");

            print!("Enter vehicle number: ");
            io::stdout().flush().unwrap();
            let mut vehicle_number = String::new();
            io::stdin().read_line(&mut vehicle_number).expect("Failed to read input");

            spot.is_occupied = true;
            spot.customer_name = Some(customer_name.trim().to_string());
            spot.vehicle_number = Some(vehicle_number.trim().to_string());
            spot.booking_time = Some(current_timestamp());

            println!("Spot {} is booked.", spot_number);
        } else {
            println!("Spot {} is already occupied.", spot_number);
        }
    } else {
        println!("Invalid spot number. Please choose a spot between 1 and 30.");
    }
}

fn release_parking_spot (parking_spots: &mut [ParkingSpot], spot_number: usize) {
    if 1 <= spot_number && spot_number <= 30 {
        let spot = &mut parking_spots[spot_number - 1];
        if spot.is_occupied {
            let booking_duration = current_timestamp() - spot.booking_time.unwrap();
            let fee = calculate_fee(booking_duration);
            println!(
                "Spot {} is released. Customer: {}, Vehicle No: {}. Duration: {} seconds. Fee: ${}",
                spot_number,
                spot.customer_name.as_ref().unwrap(),
                spot.vehicle_number.as_ref().unwrap(),
                booking_duration,
                fee
            );
            spot.is_occupied = false;
            spot.customer_name = None;
            spot.vehicle_number = None;
            spot.booking_time = None;
        } else {
            println! ("Spot {} is already empty.", spot_number);
        }
    } else {
        println!("Invalid spot number. Please choose a spot between 1 and 30.");
    }
}

fn get_spot_number(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn calculate_fee(duration: u64) -> u64 {
    // Fee calculation based on time in seconds
    let rate_per_hour = 5; // Let's say $5 per hour
    (duration as f64 / 3600.0 * rate_per_hour as f64).ceil() as u64
}
