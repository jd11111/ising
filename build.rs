use std::fs::File;
use std::io::prelude::*;
use toml::Table;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=config.toml");

    //read config file and parse it using toml crate
    let mut file = File::open("config.toml").expect("could not open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("could not read file");
    let config = contents.parse::<Table>().unwrap();

    //extract parameters from config (should have automated this...)
    let n = config["N"].as_integer().unwrap() as usize;
    let n_cycle = config["N_CYCLE"].as_integer().unwrap() as i16;
    let n_burncycles = config["N_BURNCYCLES"].as_integer().unwrap() as i16;
    let n_mag = config["N_MAG"].as_integer().unwrap() as i16;
    let n_betasamples = config["N_BETASAMPLES"].as_integer().unwrap() as i16;
    let beta_min = config["BETA_MIN"].as_float().unwrap() as f32;
    let beta_max = config["BETA_MAX"].as_float().unwrap() as f32;
    


    let y = format!("pub const N : usize = {};\npub const N_CYCLE : i16 = {};\npub const N_BURNCYCLES : i16 = {};\npub const N_MAG : i16 = {};\npub const N_BETASAMPLES : i16 = {};\npub const BETA_MIN: f32 = {:.32};\npub const BETA_MAX: f32 = {:.32};",n, n_cycle,n_burncycles,n_mag,n_betasamples,beta_min,beta_max);
   
   //write rust config to a file for import in main
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("config_in.rs");
    std::fs::write(&path, y).unwrap();
}