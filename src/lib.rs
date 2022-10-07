use std::collections::HashMap;

pub fn median(list: &mut Vec<i32>) -> f64 {

    list.sort();

    if list.len() == 0 {
        return -1.0;
    }
    if list.len() == 1 {
        return list[0] as f64;
    }

    if list.len() % 2 == 0 {
        let i1 = list.len() / 2;
        let i2 = list.len() / 2 - 1;
        let val1 = list[i1] as f64;
        let val2 = list[i2] as f64;
        return (val1 + val2) / 2.0;

    }
    if list.len() % 2 != 0 {
        return list[list.len() / 2] as f64;
    }

    return 0.0;
}

pub fn mean(list: &Vec<i32>) -> f64 {
    if list.len() == 0 {
        return -1.0;
    }
    if list.len() == 1 {
        return list[0] as f64;
    }

    let sum: i32 = list.iter().sum();
    return sum as f64 / list.len() as f64;
}

pub fn moda(list: &Vec<i32>) -> i32 {
    if list.len() == 0 {
        return -1;
    }
    if list.len() == 1 {
        return 1;
    }
     
    let mut max_key = 0;
    let mut max_val = 0;
    let mut map = HashMap::new();
    for elem in list {
        let count = map.entry(elem).or_insert(0);
        *count += 1;
    }
    for (key, val) in map {
        if val > max_val {
            max_val = val;
            max_key = *key;
        }
    }
    max_key
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn median_test() {
        let mut empty_list: Vec<i32> = vec![];
        let mut single_list: Vec<i32> = vec![1];
        let mut sorted_list_odd = vec![1, 2, 3, 4, 5, 6, 7];
        let mut unsorted_list_odd = vec![2, 5, 3, 1, 7, 9, 4];
        let mut sorted_list_even = vec![1, 2, 3, 4, 5, 6];
        let mut unsorted_list_even = vec![6, 4, 1, 3, 8, 2];

        let result = median(&mut empty_list);
        assert_eq!(result, -1.0);

        let result = median(&mut single_list);
        assert_eq!(result, 1.0);

        let result = median(&mut sorted_list_odd);
        assert_eq!(result, 4.0);

        let result = median(&mut unsorted_list_odd);
        assert_eq!(result, 4.0);

        let result = median(&mut sorted_list_even);
        assert_eq!(result, 3.5);

        let result = median(&mut unsorted_list_even);
        assert_eq!(result, 3.5);
    }

    #[test]
    fn mean_test() {
        let empty_list: Vec<i32> = vec![];
        let single_list: Vec<i32> = vec![1];
        let simple_list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    
        let res = mean(&empty_list);
        assert_eq!(res, -1.0);
    
        let res = mean(&single_list);
        assert_eq!(res, 1.0);
    
        let res = mean(&simple_list);
        assert_eq!(res, 4.0);
    }

    #[test]
    fn moda_test() {
        let empty_list: Vec<i32> = vec![];
        let single_list: Vec<i32> = vec![2];
        let simple_list: Vec<i32> = vec![1, 2, 2, 4, 3, 3, 3, 8, 6];

        let res = moda(&empty_list);
        assert_eq!(res, -1);

        let res = moda(&single_list);
        assert_eq!(res, 1);

        let res =  moda(&simple_list);
        assert_eq!(res, 3);
    }
}






