use std::{f64::consts::PI, path::Path, time::Instant};

use image::{ImageBuffer, Rgb, RgbImage};
use num_complex::Complex;
use palette::{FromColor, Lch, Srgb};
use rayon::iter::{ParallelBridge, ParallelIterator};
extern crate image;
extern crate rayon;

fn main() {
    let top = Instant::now();
    const PIX_WIDTH: u32 = 1000;
    const PIX_HEIGHT: u32 = 1000;
    const PIX_LENGTH: u32 = PIX_WIDTH * PIX_HEIGHT;
    const MAX_ITER: u32 = 10000;

    // let mut state: Vec<u32> = Vec::with_capacity(PIX_LENGTH as usize);
    let mut state_top: Vec<u32> = vec![0; (PIX_LENGTH / 2) as usize];
    let mut frame: ImageBuffer<Rgb<f64>, Vec<f64>> = ImageBuffer::new(PIX_WIDTH, PIX_HEIGHT);

    state_top.iter_mut().enumerate().par_bridge().for_each( |p| {
        let c0: Complex<f64> = Complex::new((p.0 % PIX_WIDTH as usize) as f64 / PIX_WIDTH as f64 * 2.47 - 2.0, (p.0 / PIX_WIDTH as usize) as f64 / PIX_HEIGHT as f64 * 2.12 - 1.12);
        *p.1 = mandel_der(c0, MAX_ITER);
    });

    // state.append(&mut state_top);
    // state.append(&mut state_bottom);

    state_top.iter().enumerate().for_each(|p| {
        //let mut v = ((((*p.1 as f64 / MAX_ITER as f64) as f64).powf(BIG_S) * PALETTE_COUNT as f64).powf(1.5) % (PALETTE_COUNT as f64)) as u8;
        let s = *p.1 as f64 / MAX_ITER as f64;
        let v = 1.0 - f64::cos(PI * s).powf(2.0);
        let l = 75.0 - (75.0 * v);
        let rgb = Srgb::from_color(Lch::new(l, 28.0 + l, (360.0 * s).powf(1.5) % 360.0));
        let x = (p.0 as f64 % PIX_WIDTH as f64) as u32;
        let y = (p.0 as f64 / PIX_WIDTH as f64) as u32;
        frame.put_pixel(x, y, Rgb([rgb.red, rgb.green, rgb.blue]));
        frame.put_pixel(x, PIX_HEIGHT - y - 1, Rgb([rgb.red, rgb.green, rgb.blue]));
    });

    let mut frame_final = RgbImage::new(PIX_WIDTH, PIX_HEIGHT);
    frame.enumerate_pixels().for_each(|p| {
        frame_final.put_pixel(p.0, p.1, Rgb([(p.2.0[0] * 255.0) as u8, (p.2.0[1] * 255.0) as u8, (p.2.0[2] * 255.0) as u8]));
    });

    frame_final.save(&Path::new("image.png")).unwrap();
    let bottom = Instant::now();
    dbg!(bottom - top);
}

const TWO: Complex<f64> = Complex::new(2.0, 0.0);
fn mandel_der(c0: Complex<f64>, max_iter: u32) -> u32 {
    let mut c: Complex<f64> = Complex::new(0.0, 0.0);
    let mut dc: Complex<f64> = Complex::ONE;
    let mut dc_sum: Complex<f64> = Complex::new(0.0, 0.0);

    let q: f64 = (c0.re - 0.25).powi(2) + c0.im.powi(2);
    if q * (q + (c0.re - 0.25)) <= 0.25 * c0.im.powi(2) || (c0.re + 1.0).powi(2) + c0.im.powi(2) <= 0.0625 {
        return 127;
    }

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