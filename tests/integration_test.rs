extern crate rustypi;

#[test]
fn test_pi_estimation() {
    let simulator = rustypi::RustyPi::new(1_000_000);
    let pi_estimate = simulator.estimate_pi();
    assert!((pi_estimate - 3.14).abs() < 0.05, "Pi estimation should be close to 3.14");
}
