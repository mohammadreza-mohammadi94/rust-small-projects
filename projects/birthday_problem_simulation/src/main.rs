use std::collections::HashSet;
use rand::Rng;

/// Simulates the Birthday Problem probability using Monte Carlo trials.
///
/// # Arguments
///
/// * `group_size` - The number of people in the group (e.g., 23).
/// * `trials` - The number of simulation runs (e.g., 1,000,000).
///
/// # Returns
///
/// The calculated probability of at least two people sharing a birthday as an f64.
fn birthday_simulation(group_size: usize, trials: u32) -> f64 {
    // Initialize the thread-local random number generator.
    let mut rng = rand::thread_rng();

    let mut same_birthday_count: u32 = 0;

    // Loop for the specified number of trials (simulation runs).
    for _ in 0..trials {
        // Use a vector to store the birthdays. u16 is sufficient for 1-365.
        let mut birthdays: Vec<u16> = Vec::with_capacity(group_size);
        
        // Generate random birthdays for everyone in the group.
        for _ in 0..group_size {
            // rng.gen_range generates a number in the range [1, 365] inclusive.
            let day = rng.gen_range(1..=365);
            birthdays.push(day);
        }

        // Check for duplicates efficiently using a HashSet.
        // This is analogous to checking len(birthdays) != len(set(birthdays)) in Python.
        let mut unique_birthdays: HashSet<u16> = HashSet::with_capacity(group_size);
        let mut has_duplicate = false;
        
        for day in birthdays {
            // HashSet::insert returns false if the value was already present (i.e., a duplicate).
            if !unique_birthdays.insert(day) {
                has_duplicate = true;
                break; // Optimization: we can stop checking as soon as a duplicate is found.
            }
        }
        
        if has_duplicate {
            same_birthday_count += 1;
        }
    }

    // Calculate the probability. Must explicitly cast to f64 for floating-point division.
    same_birthday_count as f64 / trials as f64
}

/// The main entry point of the application.
fn main() {
    // Define the total number of simulation runs. Explicit type u32 for clarity.
    let trials: u32 = 1_000_000;

    println!("Starting Birthday Problem simulation with {} trials...", trials);

    // List of group sizes to test. We use `&n` to iterate over the array elements.
    for &n in &[5, 10, 20, 23, 30, 50, 100] {
        // Convert the group size (n) to usize, which is the standard type for indexing/sizes in Rust.
        let n_usize: usize = n; 
        
        let probability = birthday_simulation(n_usize, trials);
        
        // Print the result, formatting the probability to four decimal places.
        // Note: Rust uses the same format string concept as Python's f-strings, but within macros.
        println!("Group of {}: Probability of shared birthday: {:.4}", n, probability);
    }
}
