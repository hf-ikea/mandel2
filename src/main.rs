use std::{f64::consts::PI, path::Path, time::Instant};

use image::{Rgb, RgbImage};
use num_complex::{Complex, ComplexFloat};
use rand_pcg::{Mcg128Xsl64, Pcg64Mcg};
use rayon::iter::{ParallelBridge, ParallelIterator};
use rand::prelude::*;
extern crate image;
extern crate rayon;

fn main() {
    let top = Instant::now();
    const PIX_WIDTH: u32 = 1000;
    const PIX_HEIGHT: u32 = 1000;
    const PIX_LENGTH: u32 = PIX_WIDTH * PIX_HEIGHT;
    const MAX_ITER: u32 = 256;
    //const SAMPLE_COUNT: usize = 4;

    //let mut state_top: Vec<f64> = vec![0.0; (PIX_LENGTH / 2) as usize];
    let mut state: Vec<f64> = vec![0.0; PIX_LENGTH as usize];

    state.iter_mut().enumerate().par_bridge().for_each( |p| {
        // let mut iters_total: f64 = 0.0;
        // for _i in 0..SAMPLE_COUNT {
        //     iters_total += mandel_der(get_sample_loc(p.0, PIX_WIDTH as usize, PIX_HEIGHT as usize, true), MAX_ITER);
        // }
        // *p.1 = iters_total / SAMPLE_COUNT as f64;
        *p.1 = mandel_der(get_sample_loc(p.0, PIX_WIDTH as usize, PIX_HEIGHT as usize, false), MAX_ITER);
    });

    println!("iter done");

    let mut frame_final = RgbImage::new(PIX_WIDTH, PIX_HEIGHT);
    state.iter().enumerate().for_each(|p| {
        let color: Rgb<u8> = get_color(*p.1);
        let (x, y) = index_to_coord(p.0, PIX_WIDTH, PIX_HEIGHT);
        frame_final.put_pixel(x, y, color);
        //frame_final.put_pixel(x, PIX_HEIGHT - y - 1, color);
    });

    frame_final.save(&Path::new("image.png")).unwrap();

    let bottom = Instant::now();
    dbg!(bottom - top);
}

const TWO: Complex<f64> = Complex::new(2.0, 0.0);
const BAILOUT: f64 = 1000000.0;
fn mandel_der(c0: Complex<f64>, max_iter: u32) -> f64 {
    let q: f64 = (c0.re - 0.25).powi(2) + c0.im.powi(2);
    if q * (q + (c0.re - 0.25)) <= 0.25 * c0.im.powi(2) || (c0.re + 1.0).powi(2) + c0.im.powi(2) <= 0.0625 {
        return 0.0;
    }

    let mut c: Complex<f64> = Complex::new(0.0, 0.0);
    let mut dc: Complex<f64> = Complex::ONE;
    let mut c_old: Complex<f64> = Complex::new(0.0, 0.0);
    let mut period: u32 = 0;
    for cur_iter in 0..=max_iter {
        c = c.powi(2) + c0;
        dc = TWO * dc * c + Complex::ONE;

        let csqr = c.norm_sqr();
        if csqr > BAILOUT {
            //return cur_iter as f64;
            //return (cur_iter as f64) - f64::log2(Complex::abs(c) / BAILOUT_POWER);
            return f64::log10(f64::log10(csqr) / f64::powf(2.0, cur_iter as f64)) / 7.0;
        }

        if c == c_old {
            return 0.0;
        }

        period += 1;
        if period > 20 {
            period = 0;
            c_old = c;
        }
    }
    return 0.0;
}

fn get_color(i: f64) -> Rgb<u8> {
    let r: f64 = 0.5 + (0.5 * f64::cos(2.0 * PI * i));
    let g: f64 = 0.5 + (0.5 * f64::cos(2.0 * PI * (i + 0.1)));
    let b: f64 = 0.5 + (0.5 * f64::cos(2.0 * PI * (i + 0.2)));
    Rgb([(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8])
}

fn rand_f64(rand: &mut Mcg128Xsl64) -> f64 {
    (rand.next_u32() / u32::MAX) as f64
}

fn index_to_coord(p: usize, width: u32, height: u32) -> (u32, u32) {
    ((p as f64 % width as f64) as u32, (p as f64 / height as f64) as u32)
}

fn get_sample_loc(p: usize, width: usize, height: usize, offset: bool) -> Complex<f64> {
    let (x, y) = index_to_coord(p, width as u32, height as u32);
    let x: f64 = x as f64 / width as f64 * 2.47 - 2.0;
    let y: f64 = y as f64 / height as f64 * 2.12 - 1.12;
    if !offset {
        return Complex::new(x, y);
    }
    let mut rand = Pcg64Mcg::new(p.try_into().unwrap());
    let x_offset: f64 = rand_f64(&mut rand) * 0.005 * x;
    let y_offset: f64 = rand_f64(&mut rand) * 0.005 * y;
    
    Complex::new(x + x_offset, y + y_offset)
}