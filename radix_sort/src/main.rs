fn get_digit(x: i32, digit: usize) -> usize {
    let str = x.to_string();
    
    if digit > str.len() { return 0; }
    str.chars().nth(str.len() - digit).unwrap().to_digit(10).unwrap() as usize
}

fn radix_sort(arr: &mut Vec<i32>) {
    let mut digit = 1;
    loop {
        let mut data: Vec<Vec<i32>> = (0..10).into_iter().map(|_| Vec::new()).collect();
        
        let mut digit_sum = 0;
        for &mut x in &mut *arr {
            let y = get_digit(x, digit);
            digit_sum += y;
            data[y].push(x);
        }

        *arr = data.into_iter().flatten().collect();
        digit += 1;

        if digit_sum == 0 { break; }
    }
}

fn main() {
    let mut arr = vec![6, 3, 123, 43, 12, 533, 87, 12, 1, 0, 9, 34, 98];
    radix_sort(&mut arr);
    println!("{:?}", arr);
}
