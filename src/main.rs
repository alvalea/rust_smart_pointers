use std::cell::RefCell;
use std::option::Option;
use std::rc::Rc;

pub struct StackNode<T: std::fmt::Display> {
    value: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T: std::fmt::Display> StackNode<T> {
    pub fn new(value: T) -> Box<Self> {
        Box::new(StackNode { value, next: None })
    }
}

impl<T: std::fmt::Display> Drop for StackNode<T> {
    fn drop(&mut self) {
        println!("Drop stacknode {}", self.value);
    }
}

pub struct Stack<T: std::fmt::Display> {
    top: Option<Box<StackNode<T>>>,
}

impl<T: std::fmt::Display> Stack<T> {
    pub fn new() -> Self {
        Self { top: None }
    }

    pub fn push(&mut self, value: T) {
        let mut new_node = StackNode::new(value);
        match self.top.take() {
            Some(old_top) => {
                new_node.next = Some(old_top);
            }
            None => {}
        }
        self.top = Some(new_node);
    }

    pub fn print(&self) {
        let mut node = &self.top;
        while let Some(current) = node {
            println!("{}", current.value);
            node = &current.next;
        }
    }
}

pub struct ListNode<T: std::fmt::Display> {
    value: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: std::fmt::Display> ListNode<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, next: None }))
    }
}

impl<T: std::fmt::Display> Drop for ListNode<T> {
    fn drop(&mut self) {
        println!("Drop listnode {}", self.value);
    }
}

pub struct List<T: std::fmt::Display> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
    tail: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: std::fmt::Display> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        let node = ListNode::new(value);
        if let Some(old_tail) = self.tail.take() {
            old_tail.borrow_mut().next = Some(node.clone());
        }
        self.tail = Some(node.clone());

        if self.head.is_none() {
            self.head = Some(node.clone());
        }
    }

    fn delete_zero(&mut self) {
        if let Some(node) = self.head.take() {
            self.head = node.borrow_mut().next.take();
            if self.head.is_none() {
                self.tail = None;
            }
        }
    }

    fn delete_non_zero(&mut self, index: usize) {
        let mut i = 0;
        let mut node = self.head.clone();
        let mut prev_node: Option<Rc<RefCell<ListNode<T>>>> = None;
        while let Some(current) = node {
            if i == index {
                if let Some(prev) = prev_node {
                    prev.borrow_mut().next = current.borrow().next.clone();
                    if prev.borrow().next.is_none() {
                        self.tail = Some(prev);
                    }
                }
                return;
            }
            prev_node = Some(current.clone());
            node = current.borrow().next.clone();
            i += 1;
        }
    }

    pub fn delete(&mut self, index: usize) {
        if index == 0 {
            self.delete_zero();
        } else {
            self.delete_non_zero(index);
        }
    }

    pub fn print(&self) {
        let mut node = self.head.clone();
        while let Some(current) = node {
            println!("{} -> ", current.borrow().value);
            node = current.borrow().next.clone();
        }
    }
}

fn test_stack() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.print();
}

fn test_list() {
    let mut list = List::new();
    list.insert(1);
    list.insert(2);
    list.insert(3);
    list.print();
    list.delete(2);
    println!("\n");
    list.print();
}

fn main() {
    test_stack();
    println!("\n");
    test_list();
}
