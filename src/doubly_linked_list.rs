use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Self {
        Node {
            key,
            value,
            prev: None,
            next: None,
        }
    }
}

pub struct DoublyLinkedListLruCache {
    capacity: usize,
    size: usize,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
    m: HashMap<i32, Rc<RefCell<Node>>>,
}

impl DoublyLinkedListLruCache {
    pub fn new(capacity: i32) -> Self {
        let mut head = Rc::new(RefCell::new(Node::new(-1, -1)));
        let mut tail = Rc::new(RefCell::new(Node::new(-1, -1)));

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        DoublyLinkedListLruCache {
            capacity: capacity as usize,
            size: 0,
            head,
            tail,
            m: HashMap::new(),
        }
    }

    fn remove_node(&mut self, node: Rc<RefCell<Node>>) -> i32 {
        let node_next = node.borrow().next.as_ref().unwrap().clone();
        let node_prev = node.borrow().prev.as_ref().unwrap().clone();

        node.borrow_mut().next = None;
        node.borrow_mut().prev = None;

        node_next.borrow_mut().prev = Some(node_prev.clone());
        node_prev.borrow_mut().next = Some(node_next);
        self.size -= 1;
        node.borrow().key
    }

    fn add_node(&mut self, key: i32, value: i32) -> Rc<RefCell<Node>> {
        let mut node = Node::new(key, value);

        let tail_prev = self.tail.borrow().prev.as_ref().unwrap().clone();

        node.next = Some(self.tail.clone());
        node.prev = Some(tail_prev.clone());

        let node_ref = Rc::new(RefCell::new(node));

        tail_prev.borrow_mut().next = Some(node_ref.clone());
        self.tail.borrow_mut().prev = Some(node_ref.clone());

        self.size += 1;
        node_ref
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if self.m.contains_key(&key) {
            let node = self.m.get(&key).unwrap().clone();
            self.remove_node(node.clone());

            let val = node.borrow().value;
            let new_node = self.add_node(key, val);
            self.m.insert(key, new_node);
            val
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.m.contains_key(&key) {
            let node = self.m.get(&key).unwrap().clone();
            self.remove_node(node.clone());

            let new_node = self.add_node(key, value);
            self.m.insert(key, new_node);
        }
        else {
            let new_node = self.add_node(key, value);
            self.m.insert(key, new_node);

            if self.size > self.capacity {
                let node_to_remove = self.head.borrow().next.as_ref().unwrap().clone();
                let key_to_remove = self.remove_node(node_to_remove);
                self.m.remove(&key_to_remove);
            }
        }
    }
}
