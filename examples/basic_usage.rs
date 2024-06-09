extern crate rustypi;

fn main() {
    let simulator = rustypi::RustyPi::new(1_000_000);
    let pi_estimate = simulator.estimate_pi();

    println!("Estimated value of Pi: {}", pi_estimate);
}
