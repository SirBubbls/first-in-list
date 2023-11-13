pub fn sort(list: &Vec<i32>) -> i32 {
    let mut temp: Vec<Option<bool>> = Vec::with_capacity(100_000_000);
    // gesucht ist minimum was nicht in liste ist
    let list = vec![8, 2, 3, 6, 9, 2, 1, 11, 5, 0];

    for i in list {
        if i > 0 {
            temp[i] = Some(true);
        }
    }

    let index = temp.iter().position(|x| x.is_none());
    index.unwrap_or(1) as i32
}
