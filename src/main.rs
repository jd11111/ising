use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;
use std::time::Instant;

const N : usize = 30; // number of lattice sites in either direction (so N^2 sites in total)
const N_CYCLE :i16 = 1000; // number of times to advance the chain between taking samples
const N_MAG : i16 = 10000;// number of times to sample the magnetization
const N_BURNCYCLES :i16 = 100; // number of cycles to initially advance the chain without sampling
const N_BETASAMPLES : i16 = 100; //number of beta values to calculate magnetization for (equally spaced)
const BETA_MIN : f32 = 0.0; // beta start value 
const BETA_MAX : f32 = 1.0;// beta end value

fn calc_exparr(beta:&f32)->[f32;9]{
    let mut out: [f32;9]= [0.0;9];
    for i in 0..9{
        let x= -2.0*beta*i as f32;
        out[i]= x.exp();
    }
    return out;
}

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
            //let x: f32 = -2.0*beta*(var as f32); //can optimize by precomputing the exps here
            if y <= x{
                state[i][j]=-1*state[i][j];
                update_boundary(state, i, j);
            }
        }
    }
}

fn calc_mag(state: &mut [[i8;N+2];N+2])->i32{
    let mut acc: i32 =0;
    for i in 1..N+1{
        for j in 1..N+1{
            acc += state[i][j] as i32;
        }
    }
    return acc.abs();
}

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
    let mut betas : Vec<_>=vec![];
    let var = (BETA_MAX-BETA_MIN)/(N_BETASAMPLES as f32);
    for i in 0.. N_BETASAMPLES+1{
        let beta = BETA_MIN+i as f32 * var;
        betas.push(beta);
    }
    let now = Instant::now();

    let par_iter = betas.par_iter().map(run);
    let mags_result: Vec<_> = par_iter.collect();

    let elapsed = now.elapsed();

    println!("Used beta values:");
    println!("{:?}", betas);
    println!("Calculated average magnetization:");
    println!("{:?}", mags_result);
    println!("Time spent simulating: {:.2?}", elapsed);
}