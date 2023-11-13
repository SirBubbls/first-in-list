#![feature(portable_simd)]
use chrono::{NaiveTime, Utc};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::simd::u32x64;

pub fn simd(vec1: &[i32], vec2: &[i32], result: &mut [i32]) {
    let mut cursor = 0;

    for _ in 0..(vec1.len() / 64) {
        let x = std::simd::i32x64::from_slice(&vec1[cursor..cursor + 64])
            - std::simd::i32x64::from_slice(&vec2[cursor..cursor + 64]);

        x.copy_to_slice(&mut result[cursor..cursor + 64]);

        cursor += 64;
    }
}
fn generate_list(array_size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let missing = rng.gen_range(0..array_size);
    let mut list: Vec<i32> = (0..array_size)
        .filter_map(|x| if x == missing { None } else { Some(x as i32) })
        .collect();
    list.shuffle(&mut rng);
    list
}

#[inline(never)]
pub fn findme(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    vec1.into_iter()
        .zip(vec2.into_iter())
        .map(|(x, y)| x - y)
        .collect_vec()
}

fn main() {
    for _ in 0..50 {
        let mut list = generate_list(5_000_000);
        list.sort_unstable();
        print!("{}", list[0]);
    }

    let lists: Vec<Vec<i32>> = (0..30).map(|_| generate_list(1_000_000)).collect_vec();
    let start_time: NaiveTime = Utc::now().time();
    for mut x in lists {
        x.sort_unstable();
    }
    let end_time: NaiveTime = Utc::now().time();
    println!("\n{:?}", (end_time - start_time).num_milliseconds() / 30);

    //let mut x = [0; 270_000];
    //let x = simd(&generate_list(1000), &generate_list(1000), &mut x);
    //let x = findme(generate_list(1000), generate_list(1000));
    //println!("{:?}", x);
}
