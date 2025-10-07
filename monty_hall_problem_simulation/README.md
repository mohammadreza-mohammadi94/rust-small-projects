# Monty Hall Problem Simulation

This repository contains a Rust program that simulates the famous Monty Hall problem. The simulation is designed to demonstrate the counter-intuitive probabilities involved and to verify the optimal strategy.

## The Monty Hall Problem

The Monty Hall problem is a brain teaser, in the form of a probability puzzle, loosely based on the American television game show *Let's Make a Deal* and named after its original host, Monty Hall.

The problem is stated as follows:
> Suppose you're on a game show, and you're given the choice of three doors: Behind one door is a car; behind the others, goats. You pick a door, say No. 1, and the host, who knows what's behind the doors, opens another door, say No. 3, which has a goat. He then says to you, "Do you want to pick door No. 2?" Is it to your advantage to switch your choice?

The counter-intuitive answer is that it is to your advantage to switch. This simulation aims to prove this by running a large number of trials.

## About this Simulation

This Rust program runs a large number of trials (1,000,000 by default) for both of the contestant's possible strategies:
1.  **STAY**: The contestant sticks with their initial choice.
2.  **SWITCH**: The contestant switches to the other unopened door.

The program calculates and prints the win probability for each strategy, demonstrating that the "SWITCH" strategy yields a win probability of approximately 2/3, while the "STAY" strategy yields a win probability of approximately 1/3.

## How to Run

1.  **Clone the repository and navigate to the project directory.**
2.  **Build the project using Cargo:**
    ```sh
    cargo build --release
    ```
3.  **Run the simulation:**
    ```sh
    cargo run --release
    ```

## Expected Output

When you run the simulation, you should see output similar to the following:

```
--- Monty Hall Simulation (1000000 Trials) ---
Strategy: STAY (Do not switch)
Win Probability: 0.3333

--------------------------

Strategy: SWITCH (Change your choice)
Win Probability: 0.6667
```

This output clearly shows that switching doors results in a significantly higher chance of winning the car.
