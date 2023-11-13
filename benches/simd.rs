#![feature(portable_simd)]

use criterion::Criterion;
use criterion::{criterion_group, criterion_main};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::simd::{u32x64, SimdInt};

fn generate_list(array_size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let missing = 1;
    let mut list: Vec<i32> = (0..array_size)
        .filter_map(|x| if x == missing { None } else { Some(x as i32) })
        .collect();
    list.shuffle(&mut rng);
    list
}

pub fn simd(vec1: &[i32], vec2: &[i32], result: &mut [i32]) {
    let mut cursor = 0;

    for _ in 0..(vec1.len() / 64 - 1) {
        let x = std::simd::i32x64::from_slice(&vec1[cursor..cursor + 64])
            .saturating_add(std::simd::i32x64::from_slice(&vec2[cursor..cursor + 64]));
        x.copy_to_slice(&mut result[cursor..cursor + 64]);

        cursor += 64;
    }
}
pub fn numpy_in_billig(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let vec1 = ndarray::Array::from_vec(vec1);

    let vec2 = ndarray::Array::from_vec(vec2);

    let result = vec2 - vec1;

    *result.get(0).unwrap()
}

pub fn iterator(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    vec1.into_iter()
        .zip(vec2.into_iter())
        .map(|(x, y)| x - y)
        .collect_vec()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let sizes = [2_000_000];

    for array_size in sizes.iter() {
        let mut group = c.benchmark_group(format!("{}", &array_size));

        let mut x = [0; 2_000_000];
        let list1 = generate_list(*array_size);
        let list2 = generate_list(*array_size);

        group.bench_with_input(format!("simd"), &(list1, list2), |b, (list1, list2)| {
            b.iter_with_large_drop(|| {
                simd(
                    list1, 
                    list2,
                    &mut x,
                )
            });
        });
        group.bench_with_input(format!("diff"), &array_size, |b, size| {
            b.iter_with_large_drop(|| iterator(generate_list(*array_size), generate_list(*array_size)));
        });

        let list1 = generate_list(*array_size);
        let list2 = generate_list(*array_size);

        group.bench_with_input(format!("numpy_in_billig"), &(list1, list2), |b, (x, y)| {
            b.iter_with_large_drop(|| numpy_in_billig(x.clone(), y.clone()));
        });
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
