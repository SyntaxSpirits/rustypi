# RustyPi

RustyPi is a Rust library designed to estimate the value of π (pi) using Monte Carlo simulations. It provides a straightforward and efficient method for performing these simulations, making it an ideal choice for educational purposes, numerical methods demonstrations, and more.

## Getting Started

### Prerequisites

Ensure you have Rust installed on your machine. If Rust is not already installed, you can install it by following the instructions on the official Rust website: https://www.rust-lang.org/tools/install.

### Installing

To use RustyPi in your project, add the following to your Cargo.toml:

```toml
[dependencies]
rustypi = "0.1.0"
```

Then, run the following command to build your project and download the RustyPi crate:

```bash
cargo build
```

## Usage

Here is a simple example of how to use RustyPi to estimate the value of π:

```rust
extern crate rustypi;

fn main() {
    let simulator = rustypi::RustyPi::new(1_000,000);
    let pi_estimate = simulator.estimate_pi();
    println!("Estimated value of Pi: {}", pi_estimate);
}
```

To run this example, save it as main.rs, and run:

```bash
cargo run
```

## Running the Tests

To run the tests included with RustyPi, execute:

```bash
cargo test
```

This will run all the unit and integration tests defined in the library.

## Contributing

Contributions to RustyPi are welcome! Here are a few ways you can help:

- Report bugs and issues
- Suggest new features or improvements
- Open a pull request with improvements to code or documentation
- Please read CONTRIBUTING.md for details on our code of conduct and the process for submitting pull requests to us.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
