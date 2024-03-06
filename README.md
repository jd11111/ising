# 2D Ising Model MCMC Integration In Rust

Computes the (average) absolute magnetization per lattice site for the (standard) 2D Ising model in thermal equilibrium for a given range of inverse temperature values.

The data to produce the following figure was obtained using this program:
<img src="mag.png" alt="drawing" width="800"/>

Uses Markov Chain Monte Carlo Integration with the Metropolis-Hastings algorithm.  
The main code running the simulation is in the main.rs file in the src directory.

## Usage

This is a learning project.
Use at your own risk.

- enter the desired parameters into the self explanatory "config.toml" file in the main directory
- type ```cargo build --release``` into the console while in the main directory to (re)-compile the program (necessary after every parameter change)
- execute the binary "ising" in the target/release directory
- after execution a file named "output" will appear in the same folder. This file contains the calculated beta and magnetization values in CSV format