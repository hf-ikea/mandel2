use std::path::Path;

use image::{Rgb, RgbImage};
use num_complex::Complex;
use rayon::iter::{ParallelBridge, ParallelIterator};
extern crate image;
extern crate rayon;

fn main() {
    const PIX_WIDTH: u32 = 1920;
    const PIX_HEIGHT: u32 = 1080;
    const PIX_LENGTH: u32 = PIX_WIDTH * PIX_HEIGHT;
    const MAX_ITER: u32 = 255;
    const PALETTE_COUNT: u32 = 255;
    const BIG_S: u32 = 3;

    // let mut state: Vec<u32> = Vec::with_capacity(PIX_LENGTH as usize);
    let mut state_top: Vec<u32> = vec![0, PIX_LENGTH / 2];
    let mut state_bottom: Vec<u32> = vec![0, PIX_LENGTH / 2];

    let mut frame = RgbImage::new(PIX_WIDTH, PIX_HEIGHT);

    state_top.iter_mut().par_bridge().for_each( |p| {
        let c0: Complex<f64> = Complex::new((*p % 1080) as f64 / 1920.0 * 2.47 - 2.0, (*p % 1920) as f64 / 1080.0 * 2.12 - 1.12);
        *p = mandel_der(c0, MAX_ITER);
    });

    state_bottom.iter_mut().enumerate().par_bridge().for_each( |p| {
        *p.1 = state_top[p.0];
    });

    // state.append(&mut state_top);
    // state.append(&mut state_bottom);

    state_top.iter().enumerate().par_bridge().for_each(|p| {
        let v = ((((*p.1 / MAX_ITER).pow(BIG_S) * PALETTE_COUNT) as f64).powf(1.5) % PALETTE_COUNT as f64) as u32;
        frame.put_pixel(1, 1, Rgb([1, 1, 1]));
    });

    // frame.enumerate_pixels_mut().filter(|p| p.1 < PIX_HEIGHT / 2).par_bridge().for_each( |p| {
    //     let c0: Complex<f64> = Complex::new(p.0 as f64 / 1920.0 * 2.47 - 2.0, p.1 as f64 / 1080.0 * 2.12 - 1.12);
    //     let cur_iter: u8 = mandel_der(c0, MAX_ITER);
    //     *p.2 = Rgb([cur_iter, cur_iter, cur_iter]);
    // });

    // let (frame_top, frame_bottom) = frame.split_at_mut(framelen);

    // for (i, mut _p) in frame_bottom.into_iter().enumerate() {
    //     _p = &mut frame_top[i];
    // }
    // frame = [frame_top, frame_bottom].concat();

    // frame.enumerate_pixels_mut().filter(|p| p.1 >= PIX_HEIGHT / 2).par_bridge().for_each( |p| {
    //     *p.2 = *frame_bottom;
    // });

    frame.save(&Path::new("image.png")).unwrap();
}
const TWO: Complex<f64> = Complex::new(2.0, 0.0);
fn mandel_der(c0: Complex<f64>, max_iter: u32) -> u32 {
    let mut c: Complex<f64> = Complex::new(0.0,0.0);
    let mut dc: Complex<f64> = Complex::ONE;
    let mut dc_sum: Complex<f64> = Complex::new(0.0,0.0);

    let q: f64 = (c0.re - 0.25).powi(2) + c0.im.powi(2);
    if q * (q + (c0.re - 0.25)) <= 0.25 * c0.im.powi(2) || (c0.re + 1.0).powi(2) + c0.im.powi(2) <= 0.0625 {
        return 0;
    }

    for cur_iter in 0..=max_iter {
        c = c.powi(2) + c0;
        dc = TWO * dc * c + Complex::ONE;
        dc_sum += dc;

        if ((c.re * c.re) + (c.im * c.im)) >= 1000000.0 {
            return cur_iter;
        }
    }
    return 0;
}