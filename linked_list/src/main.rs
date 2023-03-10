struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
    prev: Option<Box<Node<T>>>
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { head: None, tail: None }
    }

    fn add(&mut self, elem: T) {
        let mut new_node = Node { elem, next: None, prev: None };

        if self.head.is_none() {
            unsafe {
                self.head = Some(Box::from_raw(&mut new_node as *mut Node<T>));
                self.tail = Some(Box::from_raw(&mut new_node as *mut Node<T>));
            }

            return;
        }

        let mut curr = self.head.as_mut().unwrap();

        while curr.next.is_some() {
            curr = curr.next.as_mut().unwrap();
        }

        unsafe {
            new_node.prev = Some(Box::from_raw(&mut **curr as *mut Node<T>));
            curr.next = Some(Box::from_raw(&mut new_node as *mut Node<T>));

            self.tail = Some(Box::from_raw(&mut new_node as *mut Node<T>));
        }
    }

    fn delete(&mut self, index: usize) -> T {
        if self.head.is_none() {
            panic!("Empty linked list");
        }

        if index == 0 {
            let mut to_delete = self.head.take().unwrap();
            self.head = to_delete.next.take();
            self.head.as_mut().unwrap().prev = None;

            return to_delete.elem;
        }

        let mut curr = self.head.as_mut().unwrap();
    
        let mut i = 0;
        while curr.next.is_some() {
            if i + 1 == index {
                break;
            }

            curr = curr.next.as_mut().unwrap();
            i += 1;
        }

        if curr.next.is_none() {
            panic!("Element does not exist");
        }

        let mut to_delete = curr.next.take().unwrap();
        unsafe {
            curr.next = to_delete.next.take();

            if curr.next.is_some() {
                curr.next.as_mut().unwrap().prev = Some(Box::from_raw(&mut **curr as *mut Node<T>));
            } else {
                self.tail = Some(Box::from_raw(&mut **curr as *mut Node<T>));
            }
        }

        to_delete.elem
    }
}

fn main() {
    let mut list = LinkedList::new();

    list.add(1);
    list.add(2);

    list.delete(0);

    println!("{:?} {:?}", list.head.is_some(), list.tail.is_some());
}
