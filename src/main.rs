use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;

use std::time::Instant;
use std::iter::zip;
use std::io::Write;                                                                                                                                                                                                                                                                                                                  
use std::fs::File;

//module that stores constant parameters
mod configin {
    include!(concat!(env!("OUT_DIR"), "/config_in.rs"));
}

//obtain constant parameters (at compile time) from the config file (using build.rs shenanigans)
const N : usize = configin::N;
const N_CYCLE :i16 = configin::N_CYCLE;
const N_BURNCYCLES :i16 = configin::N_BURNCYCLES;
const N_MAG : i16 = configin::N_MAG;
const N_BETASAMPLES : i16 = configin::N_BETASAMPLES;
const BETA_MIN : f32= configin::BETA_MIN;
const BETA_MAX : f32 = configin::BETA_MAX;


fn calc_exparr(beta:&f32)->[f32;9]{
    let mut out: [f32;9]= [0.0;9];
    for i in 0..9{
        let x= -2.0*beta*i as f32;
        out[i]= x.exp();
    }
    return out;
}

//enforce peridioc boundary conditions:
fn update_boundary(state: &mut [[i8;N+2];N+2],i:usize,j:usize) { 
    if i ==1 {
        state[N+1][j]=state[1][j];
    }
    else if i==N{
        state[0][j]=state[N][j];
    }
    if j==1{
        state[i][N+1]=state[i][1];
    }
    else if j==N{
        state[i][0]=state[i][N];
    }
}

//advance the markov chain by N_CYCLE steps:
fn do_steps(state: &mut [[i8;N+2];N+2], rng: &mut rand::prelude::ThreadRng,between: & Uniform<usize>, exp_arr : &[f32;9]){
    for _ in 0..N_CYCLE{
        let i: usize = between.sample(rng);
        let j: usize = between.sample(rng);
        let var: i8 = state[i][j]*(state[i+1][j]+state[i-1][j]+state[i][j+1]+state[i][j-1]);
        if var <=0{
            state[i][j]=-1*state[i][j];
            update_boundary(state, i, j);
        }
        else{
            let y: f32 = rng.gen();
            let x = exp_arr[var as usize];
            if y <= x{
                state[i][j]=-1*state[i][j];
                update_boundary(state, i, j);
            }
        }
    }
}

//calculate the absolute value of the total spin in the given configuration
fn calc_mag(state: &mut [[i8;N+2];N+2])->i32{
    let mut acc: i32 =0;
    for i in 1..N+1{
        for j in 1..N+1{
            acc += state[i][j] as i32;
        }
    }
    return acc.abs();
}

//do a simulation run with given beta and return the average magnetization
fn run (beta: &f32)->f32{
    let mut rng: rand::prelude::ThreadRng =  rand::thread_rng();
    let between = Uniform::from(1.. N+1);
    let exp_arr = calc_exparr(beta);
    let mut state : [[i8;N+2];N+2] =[[-1;N+2];N+2];
    let mut z =0;
    for _ in 0..N_BURNCYCLES{
        do_steps(&mut state, &mut rng, &between, &exp_arr);
    }
    for _ in 0..N_MAG{
        do_steps(&mut state, &mut rng, &between, &exp_arr);
        z+= calc_mag(&mut state);
    }
    let out: f32 = z as f32 / (N_MAG as f32 * (N as f32).powi(2));
    return out;
}



fn main() {
    //calculate the beta values for which to simulate
    let var = (BETA_MAX-BETA_MIN)/(N_BETASAMPLES as f32);
    let fun = |i|  BETA_MIN+ i as f32*var;
    let betas:Vec<f32> = (0.. N_BETASAMPLES+1).map(fun).collect();

    let now = Instant::now();
    // run the simulations in parallel
    let par_iter = betas.par_iter().map(run);
    let mags_result: Vec<_> = par_iter.collect();
    let elapsed = now.elapsed();

    println!("Simulation finished!\nTime spent simulating: {:.2?}", elapsed);
    
    //write everything to file
    let mut f = File::create("output").expect("Unable to create file");
    write!(f,"beta,magnetization\n").expect("writing to file failed");                                                                                                    
    for (a,b) in zip(&betas,&mags_result){                                                                                                                                                                  
        write!(f,"{},{}\n",a,b).expect("writing to file failed");                                                                                                              
    }     
}