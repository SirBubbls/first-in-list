use rand::prelude::*;
use rand::Rng;

fn generate_list(array_size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let missing = rng.gen_range(0..array_size);
    let mut list: Vec<i32> = (0..array_size)
        .filter_map(|x| if x == missing { None } else { Some(x as i32) })
        .collect();
    list.shuffle(&mut rng);
    list
}

pub fn create_count_representation(input: Vec<i32>) -> Vec<bool> {
    let mut temp: Vec<bool> = vec![false; 10];

    for v in input.iter() {
        temp[*v as usize] = true;
    }

    temp
}

pub fn search_in_list(list_a: &Vec<bool>, list_b: &Vec<bool>) -> Option<usize> {
    list_a
        .iter()
        .zip(list_b)
        .map(|(a, b)| (a ^ b) & a)
        .position(|x| x)
}

#[cfg(test)]
mod tests {
    use crate::{create_count_representation, generate_list};

    #[test]
    fn generate_count_repr() {
        let a = vec![1, 4, 6];

        let a = create_count_representation(a);

        assert_eq!(a[0..7], [false, true, false, false, true, false, true]);
    }

    #[test]
    fn search() {
        let a = vec![1, 2, 3, 4, 5, 6];
        let b = vec![1, 2, 4, 5, 6];
        let a = create_count_representation(a);
        let b = create_count_representation(b);

        let result = super::search_in_list(&a, &b);
        assert_eq!(result, Some(3));
    }
    #[test]
    fn search_2() {
        let a = vec![1, 2, 3, 4, 5, 6];
        let b = vec![1, 2, 3, 4, 6];
        let a = create_count_representation(a);
        let b = create_count_representation(b);

        let result = super::search_in_list(&a, &b);
        assert_eq!(result, Some(5));
    }
}
