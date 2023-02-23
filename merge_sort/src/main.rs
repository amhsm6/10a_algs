fn merge_sort<T: Clone + PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    if n < 2 { return; }

    merge_sort(&mut arr[..n/2]);
    merge_sort(&mut arr[n/2..]);

    let mut res = Vec::new();

    let mut i = 0;
    let mut j = n / 2;
    while i < n / 2 && j < n {
        if arr[i] < arr[j] {
            res.push(arr[i].clone());
            i += 1;
        } else {
            res.push(arr[j].clone());
            j += 1;
        }
    }

    if i < n / 2 {
        res.extend_from_slice(&arr[i..n/2]);
    }

    if j < n {
        res.extend_from_slice(&arr[j..]);
    }

    arr.clone_from_slice(&res);
}

fn main() {
    let mut arr = vec![1, 5, 6, 2, 8, 6, 1, 9, 6, 6, 4, 3, 50, 90, 3, 4, -123,-2, 10, 10, 10, 1231, -23];

    merge_sort(&mut arr);

    println!("{arr:?}");
}
