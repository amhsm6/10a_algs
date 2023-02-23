use std::collections::HashMap;
use std::hash::Hash;

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { head: None, }
    }
}

struct Element<T> {
    payload: T,
    priority: i32,
}

struct PQueue<T, P> {
    elements: LinkedList<Element<T>>,
    priority_arr: HashMap<P, i32>,
}

impl<T, P: Ord + Hash> PQueue<T, P> {
    fn new() -> PQueue<T, P> {
        PQueue { elements: LinkedList::new(), priority_arr: HashMap::new(), }
    }

    fn push(&mut self, elem: T, priority: P) {
        if !self.priority_arr.contains_key(&priority) {
            return
        }

        let priority = self.priority_arr[&priority];

        let mut curr = &mut self.elements.head;

        if curr.is_none() {
            *curr = Some(Box::new(Node {
                elem: Element { payload: elem, priority },
                next: None,

            }));

            return;
        }

        if curr.as_ref().unwrap().elem.priority > priority {
            *curr = Some(Box::new(Node {
                elem: Element { payload: elem, priority },
                next: curr.take(),
            }));

            return;
        }

        while curr.as_ref().unwrap().next.is_some() {
            if curr.as_ref().unwrap()
                    .next.as_ref().unwrap()
                    .elem.priority > priority
            {
                break
            }

            curr = &mut curr.as_mut().unwrap().next;
        }

        let new_node = Node {
            elem: Element { payload: elem, priority },
            next: curr.as_mut().unwrap().next.take(),
        };

        curr.as_mut().unwrap().next = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Element<T> {
        if self.elements.head.is_none() {
            panic!("Trying to pop from empty PQueue");
        }

        let new_head = self.elements.head.as_mut().unwrap().next.take();
        let elem = self.elements.head.take().unwrap().elem;

        self.elements.head = new_head;

        elem
    }
}

fn main() {
    let mut pqueue = PQueue::new();

    pqueue.priority_arr.insert("pr1", 0);
    pqueue.priority_arr.insert("pr2", 1);
    pqueue.priority_arr.insert("pr3", 2);
    pqueue.priority_arr.insert("pr4", -1);

    pqueue.push("elem1", "pr1");
    pqueue.push("elem3", "pr3");
    pqueue.push("elem2", "pr2");
    pqueue.push("elem1", "pr1");
    pqueue.push("elem4", "pr4");
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
    println!("{}", pqueue.pop().payload);
}
