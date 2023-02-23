fn quick_sort<T: Clone + PartialOrd>(arr: &mut [T]) {
    if arr.len() < 2{ return; }

    let pivot = &arr[arr.len() / 2];

    let mut le = Vec::new();
    let mut eq = Vec::new();
    let mut gt = Vec::new();

    for el in &*arr {
        if el < pivot {
            le.push(el.clone());
        } else if el > pivot {
            gt.push(el.clone());
        } else {
            eq.push(el.clone());
        }
    }

    quick_sort(&mut le);
    quick_sort(&mut gt);

    for i in 0..le.len() {
        arr[i] = le[i].clone();
    }

    for i in 0..eq.len() {
        arr[i + le.len()] = eq[i].clone();
    }

    for i in 0..gt.len() {
        arr[i + le.len() + eq.len()] = gt[i].clone();
    }
}

fn main() {
    let mut arr = vec![5, 2, 3, 8, 6, 1, 1, 2, 3, 9, 9, 10, 10];
    quick_sort(&mut arr);
    println!("{arr:?}");
}
