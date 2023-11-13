#![feature(portable_simd)]
use voracious_radix_sort::{RadixSort};
use counting_sort::CountingSort;

use pprof::criterion::{Output, PProfProfiler};
use rand::seq::SliceRandom;
use std::collections::HashSet;

use criterion::{criterion_group, criterion_main};
use criterion::{BatchSize, Criterion};
use rand::Rng;

#[inline(always)]
pub fn sorting(mut nums: Vec<i32>) -> i32 {
    glidesort::sort(&mut nums);
    nums.into_iter().position(|x| x > 1).unwrap() as i32
}

#[inline(always)]
pub fn internet_solution(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<_> = nums.iter().cloned().collect();
    (1..).find(|x| !num_set.contains(x)).unwrap_or(1)
}

#[inline(always)]
pub fn search(list: &Vec<i32>) -> i32 {
    let mut temp: Vec<Option<bool>> = vec![None; 10_000_000];
    list.iter().for_each(|i| temp[*i as usize] = Some(true));
    temp.iter().position(|x| x.is_none()).unwrap_or(1) as i32
}

fn generate_list(array_size: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let missing = rng.gen_range(0..array_size);
    let mut list: Vec<u32> = (0..array_size)
        .filter_map(|x| if x == missing { None } else { Some(x as u32) })
        .collect();
    list.shuffle(&mut rng);
    list
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let sizes = [1_000_000];

    for array_size in sizes.iter() {
        let mut group = c.benchmark_group(format!("sort unstable {}", &array_size));

        group.bench_function("unstable", move |b| {
            b.iter_batched(|| generate_list(*array_size), |mut x| x.sort_unstable(), BatchSize::LargeInput);
        });
        group.bench_function("stable", move |b| {
            b.iter_batched(|| generate_list(*array_size), |mut x| x.sort(), BatchSize::LargeInput);
        });

        group.bench_function("counting", move |b| {
            b.iter_batched(|| generate_list(*array_size), |x| x.iter().cnt_sort_min_max(&0, &5_000_000), BatchSize::LargeInput);
        });

        group.bench_function("radix", move |b| {
            b.iter_batched(|| generate_list(*array_size), |mut x| x.voracious_sort(), BatchSize::LargeInput);
        });

        group.bench_function("radix mt", move |b| {
            b.iter_batched(|| generate_list(*array_size), |mut x| x.voracious_mt_sort(8), BatchSize::LargeInput);
        });

        group.bench_function("glidesort", move |b| {
            b.iter_batched(|| generate_list(*array_size), |mut x| glidesort::sort(&mut x), BatchSize::LargeInput);
        });
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
