use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const NUM_SPOTS: usize = 30; // Total number of parking spots

#[derive(Clone)] // Add Clone trait to make ParkingSpot cloneable
struct ParkingSpot {
    occupied: bool,
    customer_name: Option<String>,
    vehicle_number: Option<String>,
    reserved_at: Option<u64>, // Reservation time in seconds since UNIX_EPOCH
}

impl ParkingSpot {
    fn new() -> Self {
        ParkingSpot {
            occupied: false,
            customer_name: None,
            vehicle_number: None,
            reserved_at: None,
        }
    }

    fn reserve(&mut self, customer_name: String, vehicle_number: String) {
        self.occupied = true;
        self.customer_name = Some(customer_name);
        self.vehicle_number = Some(vehicle_number);
        self.reserved_at = Some(current_timestamp());
    }

    fn release(&mut self) -> Option<u64> {
        if let Some(start_time) = self.reserved_at {
            self.occupied = false;
            self.customer_name = None;
            self.vehicle_number = None;
            self.reserved_at = None;
            return Some(current_timestamp() - start_time);
        }
        None
    }
}
fn main() {
    let mut parking_spots: Vec<ParkingSpot> = vec![ParkingSpot::new(); NUM_SPOTS];

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
            "2" => book_parking_spot(&mut parking_spots),
            "3" => release_parking_spot(&mut parking_spots),
            "4" => {
                println!("Exiting the parking system.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

fn display_parking_spots(parking_spots: &[ParkingSpot]) {
    println!("\nParking spots:");
    for (i, spot) in parking_spots.iter().enumerate() {
        let status = if spot.occupied {
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
    let spot_number = get_spot_number("Enter the spot number to book: ");
    if let Some(spot) = parking_spots.get_mut(spot_number - 1) {
        if !spot.occupied {
            print!("Enter customer name: ");
            io::stdout().flush().unwrap();
            let mut customer_name = String::new();
            io::stdin().read_line(&mut customer_name).expect("Failed to read input");

            print!("Enter vehicle number: ");
            io::stdout().flush().unwrap();
            let mut vehicle_number = String::new();
            io::stdin().read_line(&mut vehicle_number).expect("Failed to read input");

            spot.reserve(customer_name.trim().to_string(), vehicle_number.trim().to_string());
            println!("Spot {} is booked.", spot_number);
        } else {
            println!("Spot {} is already occupied.", spot_number);
        }
    } else {
        println!("Invalid spot number. Please choose a spot between 1 and {}.", NUM_SPOTS);
    }
}

fn release_parking_spot (parking_spots: &mut [ParkingSpot], spot_number: usize) {
    let spot_number = get_spot_number("Enter the spot number to release: ");
    if let Some(spot) = parking_spots.get_mut(spot_number - 1) {
        if spot.occupied {
            if let Some(duration) = spot.release() {
                let fee = calculate_fee(duration);
                println!(
                    "Spot {} released. Duration: {} seconds. Fee: ${}",
                    spot_number, duration, fee
                );
            }
        } else {
            println!("Spot {} is already empty.", spot_number);
        }
    } else {
        println!("Invalid spot number. Please choose a spot between 1 and {}.", NUM_SPOTS);
    }
}

fn get_spot_number(prompt: &str) -> usize {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        if let Ok(num) = input.trim().parse::<usize>() {
            if num >= 1 && num <= NUM_SPOTS {
                return num;
            }
        }
        println!("Invalid input. Please enter a number between 1 and {}.", NUM_SPOTS);
    }
}


fn current_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn calculate_fee(duration: u64) -> u64 {
    const RATE_PER_HOUR: u64 = 5;
    (duration as f64 / 3600.0 * RATE_PER_HOUR as f64).ceil() as u64
}
