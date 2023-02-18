fn bubble_sort<T: Ord, F: Fn(&T, &T) -> bool>(arr: &mut [T], func: F) {
    loop {
        let mut made_permutation = false;

        for i in 0..arr.len() - 1 {
            if func(&arr[i], &arr[i + 1]) {
                arr.swap(i, i + 1);
                made_permutation = true;
            }
        }

        if !made_permutation { break; }
    }
}

fn main() {
    let mut arr = vec![3, 1, 2, 4];
    bubble_sort(&mut arr, |&a, &b| a < b);
    println!("{:?}", arr);
}
