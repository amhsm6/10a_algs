use std::collections::HashMap;
use std::hash::Hash;

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn add(&mut self, elem: T) {
        let new = Some(Box::new(Node {
            elem,
            next: None
        }));

        if self.head.is_none() {
            self.head = new;

            return;
        }

        let mut curr = self.head.as_mut().unwrap();

        while curr.next.is_some() {
            curr = curr.next.as_mut().unwrap();
        }

        curr.next = new;
    }
}

struct Element<T> {
    payload: T,
    priority: i32
}

struct PQueue<T, P> {
    elements: LinkedList<Element<T>>,
    priority_arr: HashMap<P, i32>
}

impl<T, P: Ord + Hash> PQueue<T, P> {
    fn new() -> PQueue<T, P> {
        PQueue { elements: LinkedList::new(), priority_arr: HashMap::new() }
    }

    fn push(&mut self, elem: T, priority: P) {
        self.elements.add(Element {
            payload: elem,
            priority: self.priority_arr[&priority]
        });
    }

    fn pop(&mut self) -> Element<T> {
        if self.elements.head.is_none() {
            panic!("Trying to pop from empty PQueue");
        }

        let mut curr = self.elements.head.as_mut().unwrap();
        let mut min = &mut **curr as *mut Node<Element<T>>;

        let mut min_priority = curr.elem.priority;
        while curr.next.is_some() {
            if curr.next.as_ref().unwrap().elem.priority < min_priority {
                min_priority = curr.next.as_ref().unwrap().elem.priority;
                min = &mut **curr as *mut Node<Element<T>>;
            }

            curr = curr.next.as_mut().unwrap();
        }

        let min = unsafe { min.as_mut().unwrap() };
        let mut res = min.next.take().unwrap();
        min.next = res.next.take();

        res.elem
    }
}

fn main() {
    let mut pqueue = PQueue::new();

    pqueue.priority_arr.insert("pr1", 0);
    pqueue.priority_arr.insert("pr2", 1);
    pqueue.priority_arr.insert("pr3", 2);
    pqueue.priority_arr.insert("pr-1", -1);

    pqueue.push("elem3", "pr3");
    pqueue.push("elem2", "pr2");
    pqueue.push("elem1", "pr1");
    pqueue.push("elem-1", "pr-1");
    pqueue.push("elem3", "pr3");
    pqueue.push("elem1", "pr1");
    pqueue.push("elem3", "pr3");
    pqueue.push("elem3", "pr3");
    pqueue.push("elem3", "pr3");
    pqueue.push("elem3", "pr3");
    pqueue.push("elem3", "pr3");
    pqueue.push("elem3", "pr3");

    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
    println!("{}", pqueue.pop().payload);
}
