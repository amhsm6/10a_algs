fn heap_sort<T: Clone + PartialOrd>(arr: &[T]) -> Vec<T> {
    let mut res = Vec::new();

    let mut heap = build_heap(arr);
    while heap.len() > 0 {
        res.push(heap[0].clone());
        heap = build_heap(&heap[1..]);
    }

    res
}

fn build_heap<T: Clone + PartialOrd>(arr: &[T]) -> Vec<T> {
    let mut heap = Vec::new();

    for el in arr {
        heap.push(el.clone());

        let mut c = heap.len() - 1;
        while c != 0 {
            let curr = c;
            let parent = (c - 1) / 2;
            c = (c - 1) / 2;

            if heap[curr] >= heap[parent] {
                break;
            }

            heap.swap(curr, parent);
        }
    }

    heap
}

fn main() {
    let arr = vec![5, 2, 3, 4, 8, 7, 1, 9, 1, 1, 5, 2, 2, 9, 5];
    let res = heap_sort(&arr);
    println!("{res:?}");
}
