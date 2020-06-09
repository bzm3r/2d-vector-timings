// Copyright © 2020 Brian Merchant.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;

const NLONG: usize = 10000;
const NSHORT: usize = 10000;

fn gen_vov() -> Vec<Vec<f32>> {
    (0..NLONG)
        .map(|_| (0..NLONG).map(|y| y as f32).collect::<Vec<f32>>())
        .collect()
}

fn gen_voba() -> Vec<Box<[f32; NLONG]>> {
    (0..NLONG)
        .map(|_| {
            let mut x = Box::new([0.0_f32; NLONG]);
            x.iter_mut().enumerate().for_each(|(i, e)| *e = i as f32);
            x
        })
        .collect()
}

fn gen_voa() -> Vec<[f32; NLONG]> {
    (0..NLONG)
        .map(|_| {
            let mut x = [0.0_f32; NLONG];
            x.iter_mut().enumerate().for_each(|(i, e)| *e = i as f32);
            x
        })
        .collect()
}

fn avg_and_std(xs: &[u128]) -> (f64, f64) {
    let avg = ((xs.iter().sum::<u128>() as f64) / (xs.len() as f64)).round();
    let std = (xs
        .iter()
        .map(|&x| ((x as f64) - avg).powi(2))
        .sum::<f64>()
        / (xs.len() as f64))
        .sqrt().round();

    (std, avg)
}

fn main() {
    let mut rng = rand::thread_rng();

    let between = Uniform::from(0..NLONG);
    let mut ix_long: [usize; NLONG] = [0; NLONG];
    ix_long
        .iter_mut()
        .for_each(|e| *e = between.sample(&mut rng));

    let mut ix_short: [usize; NSHORT] = [0; NSHORT];
    ix_short
        .iter_mut()
        .for_each(|e| *e = between.sample(&mut rng));

    {
        let vov = gen_vov();
        let mut timings = vec![];
        for &i in ix_long.iter() {
            for &j in ix_short.iter() {
                let now = Instant::now();
                let _x = (&vov[i])[j];
                let t = now.elapsed().as_nanos();
                timings.push(t);
            }
        }
        let (avg, std) = avg_and_std(&timings);
        println!("vector of vector fetch ({} x {}): {} +/- {} ns", NLONG, NSHORT, avg, std,);
    }

    {
        let voa = gen_voa();
        let mut timings = vec![];
        for &i in ix_long.iter() {
            for &j in ix_short.iter() {
                let now = Instant::now();
                let _x = (&voa[i])[j];
                let t = now.elapsed().as_nanos();
                timings.push(t);
            }
        }
        let (avg, std) = avg_and_std(&timings);
        println!("vector of array fetch ({} x {}): {} +/- {} ns", NLONG, NSHORT, avg, std,);
    }

    {
        let vov = gen_voba();
        let mut timings = vec![];
        for &i in ix_long.iter() {
            for &j in ix_short.iter() {
                let now = Instant::now();
                let _x = (&vov[i])[j];
                let t = now.elapsed().as_nanos();
                timings.push(t);
            }
        }
        let (avg, std) = avg_and_std(&timings);
        println!("vector of box array fetch ({} x {}): {} +/- {} ns", NLONG, NSHORT, avg, std,);
    }
}
