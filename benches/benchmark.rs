#![feature(portable_simd)]

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

fn generate_list(array_size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let missing = rng.gen_range(0..array_size);
    let mut list: Vec<i32> = (0..array_size)
        .filter_map(|x| if x == missing { None } else { Some(x as i32) })
        .collect();
    list.shuffle(&mut rng);
    list
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let sizes = [100_000, 1_000_000, 5_000_000];

    for array_size in sizes.iter() {
        let mut group = c.benchmark_group(format!("{}", &array_size));

        let list = generate_list(*array_size);
        group.bench_function("sorting", move |b| {
            b.iter_batched(|| list.clone(), |x| sorting(x), BatchSize::LargeInput);
        });

        let list = generate_list(*array_size);
        group.bench_with_input("ram", &list, |b, size| {
            b.iter_with_large_drop(|| search(size))
        });

        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
//criterion_group! {
//name = benches;
//config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
//targets = criterion_benchmark
//}
criterion_main!(benches);
