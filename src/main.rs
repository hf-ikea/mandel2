use std::{f64::consts::PI, path::Path, time::Instant};

use image::{Rgb, RgbImage};
use num_complex::Complex;
use palette::{FromColor, Lch, Srgb};
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
    const MAX_ITER: u32 = 255;
    const SAMPLE_COUNT: usize = 4;

    let mut state_top: Vec<u32> = vec![0; (PIX_LENGTH / 2) as usize];

    state_top.iter_mut().enumerate().par_bridge().for_each( |p| {
        let mut iters_total: u32 = 0;
        for _i in 0..SAMPLE_COUNT {
            iters_total += mandel_der(get_sample_loc(p.0, PIX_WIDTH as usize, PIX_HEIGHT as usize, true), MAX_ITER);
        }
        *p.1 = iters_total / SAMPLE_COUNT as u32;
    });

    let mut frame_final = RgbImage::new(PIX_WIDTH, PIX_HEIGHT);
    state_top.iter().enumerate().for_each(|p| {
        let s = *p.1 as f64 / MAX_ITER as f64;
        let v = 1.0 - f64::cos(PI * s).powf(2.0);
        let l = 75.0 - (75.0 * v);
        let rgb = Srgb::from_color(Lch::new(l, 28.0 + l, (360.0 * s).powf(1.5) % 360.0));
        let x = (p.0 as f64 % PIX_WIDTH as f64) as u32;
        let y = (p.0 as f64 / PIX_WIDTH as f64) as u32;
        let color: Rgb<f64> = Rgb([rgb.red, rgb.green, rgb.blue]);
        let color_final: Rgb<u8> = Rgb([(color.0[0] * 255.0) as u8, (color.0[1] * 255.0) as u8, (color.0[2] * 255.0) as u8]);
        frame_final.put_pixel(x, y, color_final);
        frame_final.put_pixel(x, PIX_HEIGHT - y - 1, color_final);
    });

    frame_final.save(&Path::new("image.png")).unwrap();
    
    let bottom = Instant::now();
    dbg!(bottom - top);
}

const TWO: Complex<f64> = Complex::new(2.0, 0.0);
fn mandel_der(c0: Complex<f64>, max_iter: u32) -> u32 {
    let q: f64 = (c0.re - 0.25).powi(2) + c0.im.powi(2);
    if q * (q + (c0.re - 0.25)) <= 0.25 * c0.im.powi(2) || (c0.re + 1.0).powi(2) + c0.im.powi(2) <= 0.0625 {
        return 127;
    }

    let mut c: Complex<f64> = Complex::new(0.0, 0.0);
    let mut dc: Complex<f64> = Complex::ONE;
    let mut dc_sum: Complex<f64> = Complex::new(0.0, 0.0);
    let mut c_old: Complex<f64> = Complex::new(0.0, 0.0);
    let mut period: u32 = 0;
    for cur_iter in 0..=max_iter {
        c = c.powi(2) + c0;
        dc = TWO * dc * c + Complex::ONE;
        dc_sum += dc;

        if ((c.re * c.re) + (c.im * c.im)) >= 1000000.0 {
            return cur_iter;
        }

        if c == c_old {
            return max_iter;
        }

        period += 1;
        if period > 20 {
            period = 0;
            c_old = c;
        }
    }
    return 0;
}

fn rand_f64(rand: &mut Mcg128Xsl64) -> f64 {
    (rand.next_u32() / u32::MAX) as f64
}

fn get_sample_loc(p: usize, pix_width: usize, pix_height: usize, offset: bool) -> Complex<f64> {
    let x: f64 = (p % pix_width) as f64 / pix_width as f64 * 2.47 - 2.0;
    let y: f64 = (p / pix_width) as f64 / pix_height as f64 * 2.12 - 1.12;
    if !offset {
        return Complex::new(x, y);
    }
    let mut rand = Pcg64Mcg::new(p.try_into().unwrap());
    let x_offset: f64 = rand_f64(&mut rand) * 0.005 * x;
    let y_offset: f64 = rand_f64(&mut rand) * 0.005 * y;
    
    Complex::new(x + x_offset, y + y_offset)
}