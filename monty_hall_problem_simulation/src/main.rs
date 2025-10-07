use rand::Rng;
use std::collections::HashSet;

// ----------------------------------------------------------------------
// ENUMS AND CONSTANTS
// ----------------------------------------------------------------------

/// Represents the state of each of the three doors.
/// The prize is always behind one door (Car). The other two have Goats.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Prize {
    Car,
    Goat,
}

const NUM_DOORS: u8 = 3;

// ----------------------------------------------------------------------
// GAME LOGIC
// ----------------------------------------------------------------------

/// Runs a single trial of the Monty Hall game.
///
/// # Arguments
/// * `strategy` - Whether the contestant decides to "Switch" or "Stay".
/// * `rng` - A mutable reference to the random number generator.
///
/// # Returns
/// * `bool` - True if the contestant wins the Car, False otherwise.
fn run_monty_hall_trial<R: Rng>(strategy: bool, rng: &mut R) -> bool {
    // 1. Initialize the doors and prize location
    
    // The prize (Car) is placed randomly behind one of the three doors (0, 1, or 2).
    let car_door: u8 = rng.gen_range(0..NUM_DOORS);

    // 2. Contestant's initial choice
    // The contestant picks a door randomly (0, 1, or 2).
    let initial_choice: u8 = rng.gen_range(0..NUM_DOORS);

    // 3. Monty opens a door
    
    // Find the doors that Monty *cannot* open:
    // a) The door the contestant chose.
    // b) The door with the car.
    let mut forbidden_doors: HashSet<u8> = HashSet::new();
    forbidden_doors.insert(initial_choice);
    forbidden_doors.insert(car_door);

    // Monty must open a door that is a Goat AND is not the initial choice.
    // We iterate through all doors and find one that is allowed to be opened.
    let mut monty_opens_door: u8 = 0;
    for door in 0..NUM_DOORS {
        if !forbidden_doors.contains(&door) {
            monty_opens_door = door;
            break;
        }
    }
    // Edge Case: If the initial choice IS the Car door, Monty can open either of the two Goat doors.
    // We handle this by letting the iterator break, which works implicitly because the forbidden set 
    // will only contain two doors (initial_choice and car_door=initial_choice), leaving one choice for Monty.
    
    // 4. Contestant decides (Switch or Stay)
    let final_choice: u8 = if strategy {
        // STRATEGY: SWITCH
        // Find the remaining door (the one not initially chosen AND not opened by Monty).
        
        let mut remaining_door: u8 = 0;
        for door in 0..NUM_DOORS {
            if door != initial_choice && door != monty_opens_door {
                remaining_door = door;
                break;
            }
        }
        remaining_door
    } else {
        // STRATEGY: STAY
        initial_choice
    };

    // 5. Determine the result
    final_choice == car_door
}

// ----------------------------------------------------------------------
// SIMULATION
// ----------------------------------------------------------------------

/// Runs the full simulation for a given strategy and number of trials.
fn simulate_strategy(strategy: bool, trials: u32) -> f64 {
    let mut rng = rand::thread_rng();
    let mut wins: u32 = 0;

    for _ in 0..trials {
        if run_monty_hall_trial(strategy, &mut rng) {
            wins += 1;
        }
    }

    // Calculate probability (explicit casting needed)
    wins as f64 / trials as f64
}

fn main() {
    let trials: u32 = 1_000_000;
    println!("--- Monty Hall Simulation ({} Trials) ---", trials);

    // 1. Simulate the STAY strategy
    let prob_stay = simulate_strategy(false, trials);
    println!("Strategy: STAY (Do not switch)");
    println!("Win Probability: {:.4}", prob_stay);
    // Expected result: ~1/3 or 0.3333

    println!("\n--------------------------\n");

    // 2. Simulate the SWITCH strategy
    let prob_switch = simulate_strategy(true, trials);
    println!("Strategy: SWITCH (Change your choice)");
    println!("Win Probability: {:.4}", prob_switch);
    // Expected result: ~2/3 or 0.6666
}