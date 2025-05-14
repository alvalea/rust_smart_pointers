use std::cell::RefCell;
use std::fmt::Debug;
use std::option::Option;
use std::rc::{Rc, Weak};

struct StackNode<T: Debug> {
    value: T,
    next: Option<Box<StackNode<T>>>,
}

impl<T: Debug> StackNode<T> {
    fn new(value: T) -> Box<Self> {
        Box::new(StackNode { value, next: None })
    }
}

impl<T: Debug> Drop for StackNode<T> {
    fn drop(&mut self) {
        println!("Drop stacknode {:?}", self.value);
    }
}

struct Stack<T: Debug> {
    top: Option<Box<StackNode<T>>>,
}

impl<T: Debug> Stack<T> {
    fn new() -> Self {
        Self { top: None }
    }

    fn push(&mut self, value: T) {
        let mut new_node = StackNode::new(value);
        match self.top.take() {
            Some(old_top) => {
                new_node.next = Some(old_top);
            }
            None => {}
        }
        self.top = Some(new_node);
    }

    fn print(&self) {
        let mut node = &self.top;
        while let Some(current) = node {
            println!("{:?}", current.value);
            node = &current.next;
        }
    }
}

struct ListNode<T: Debug> {
    value: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: Debug> ListNode<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, next: None }))
    }
}

impl<T: Debug> Drop for ListNode<T> {
    fn drop(&mut self) {
        println!("Drop listnode {:?}", self.value);
    }
}

struct List<T: Debug> {
    head: Option<Rc<RefCell<ListNode<T>>>>,
    tail: Option<Rc<RefCell<ListNode<T>>>>,
}

impl<T: Debug> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn insert(&mut self, value: T) {
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

    fn delete(&mut self, index: usize) {
        if index == 0 {
            self.delete_zero();
        } else {
            self.delete_non_zero(index);
        }
    }

    fn print(&self) {
        let mut node = self.head.clone();
        while let Some(current) = node {
            println!("{:?} -> ", current.borrow().value);
            node = current.borrow().next.clone();
        }
    }
}

struct TreeNode<T: Debug> {
    value: T,
    parent: Weak<RefCell<TreeNode<T>>>,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Debug> TreeNode<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            parent: Weak::new(),
            left: None,
            right: None,
        }))
    }

    fn insert(node: &Rc<RefCell<Self>>, value: T)
    where
        T: PartialOrd,
    {
        let mut node_mut = node.borrow_mut();
        if value < node_mut.value {
            match &node_mut.left {
                None => {
                    let new_node = TreeNode::new(value);
                    new_node.borrow_mut().parent = Rc::downgrade(node);
                    node_mut.left = Some(new_node);
                }
                Some(left_node) => TreeNode::insert(left_node, value),
            }
        } else {
            match &node_mut.right {
                None => {
                    let new_node = TreeNode::new(value);
                    new_node.borrow_mut().parent = Rc::downgrade(node);
                    node_mut.right = Some(new_node);
                }
                Some(right_node) => TreeNode::insert(right_node, value),
            }
        }
    }

    fn delete(node: &Rc<RefCell<Self>>, value: T) -> bool
    where
        T: PartialOrd,
    {
        let mut node_mut = node.borrow_mut();
        if node_mut.value == value {
            node_mut.left = None;
            node_mut.right = None;
            return true;
        } else if node_mut.value > value {
            if let Some(left_node) = &node_mut.left {
                if TreeNode::delete(left_node, value) {
                    node_mut.left = None;
                }
            }
        } else if node_mut.value < value {
            if let Some(right_node) = &node_mut.right {
                if TreeNode::delete(right_node, value) {
                    node_mut.right = None;
                }
            }
        }
        return false;
    }

    fn print(node: &Rc<RefCell<Self>>) {
        let current_node = node.borrow();
        println!("{:?} ", current_node.value);
        if let Some(left_node) = &current_node.left {
            TreeNode::print(left_node);
        }
        if let Some(right_node) = &current_node.right {
            TreeNode::print(right_node);
        }
    }
}

impl<T: Debug> Drop for TreeNode<T> {
    fn drop(&mut self) {
        println!("Drop treenode {:?}", self.value);
    }
}

struct Tree<T: Debug> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: Debug> Tree<T> {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: T)
    where
        T: PartialOrd,
    {
        match &self.root {
            None => self.root = Some(TreeNode::new(value)),
            Some(root_node) => TreeNode::insert(root_node, value),
        }
    }

    fn delete(&mut self, value: T)
    where
        T: PartialOrd,
    {
        if let Some(root_node) = &self.root {
            if root_node.borrow().value == value {
                self.root = None;
            } else {
                TreeNode::delete(root_node, value);
            }
        }
    }

    fn print(&self) {
        if let Some(root_node) = &self.root {
            TreeNode::print(root_node)
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

fn test_tree() {
    let mut tree = Tree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);
    tree.print();
    println!("Before delete\n");
    tree.delete(3);
    println!("After delete\n");
    tree.print();
}

mod book {

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> Self {
            Self(x)
        }
    }

    use std::ops::{Deref, DerefMut};

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl<T> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Dropping MyBox with data");
        }
    }

    pub fn test_deref() {
        let x = 5;
        let mut y = MyBox::new(x);
        println!("{}", *y);
        *y = 7;
        println!("{}", *y);
    }

    pub fn test_drop() {
        let c = MyBox::new(String::from("some data"));
        println!("MyBox created.");
        drop(c);
        println!("MyBox dropped before the end of the function.");
    }

    #[derive(Debug)]
    enum List {
        Node(i32, Box<List>),
        Nil,
    }

    use crate::book::List::{Nil, Node};

    pub fn test_list() {
        let list = Node(1, Box::new(Node(2, Box::new(Node(3, Box::new(Nil))))));
        println!("{:?}", list);
    }

    pub mod shared_list {
        #[derive(Debug)]
        enum List {
            Node(i32, Rc<List>),
            Nil,
        }

        use crate::book::shared_list::List::{Nil, Node};
        use std::rc::Rc;

        pub fn test_shared_list() {
            let a = Rc::new(Node(5, Rc::new(Node(10, Rc::new(Nil)))));
            println!("count after creating a = {}", Rc::strong_count(&a));
            let b = Node(3, Rc::clone(&a));
            println!("count after creating b = {}", Rc::strong_count(&a));
            {
                let c = Node(4, Rc::clone(&a));
                println!("count after creating c = {}", Rc::strong_count(&a));
            }
            println!("count after c goes out of scope = {}", Rc::strong_count(&a));
        }
    }

    pub mod mut_shared_list {
        #[derive(Debug)]
        enum List {
            Node(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use crate::book::mut_shared_list::List::{Nil, Node};
        use std::cell::RefCell;
        use std::rc::Rc;

        pub fn test_mut_shared_list() {
            let value = Rc::new(RefCell::new(5));

            let a = Rc::new(Node(Rc::clone(&value), Rc::new(Nil)));

            let b = Node(Rc::new(RefCell::new(3)), Rc::clone(&a));
            let c = Node(Rc::new(RefCell::new(4)), Rc::clone(&a));

            *value.borrow_mut() += 10;

            println!("a after = {a:?}");
            println!("b after = {b:?}");
            println!("c after = {c:?}");
        }
    }

    pub mod leak {
        #[derive(Debug)]
        struct MyStruct {
            data: i32,
        }

        impl Drop for MyStruct {
            fn drop(&mut self) {
                println!("Drop MyStruct {}", self.data);
            }
        }

        #[derive(Debug)]
        enum List {
            Node(MyStruct, RefCell<Rc<List>>),
            Nil,
        }

        use crate::book::leak::List::{Nil, Node};
        use std::cell::RefCell;
        use std::rc::Rc;

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Node(_, item) => Some(item),
                    Nil => None,
                }
            }
        }

        pub fn test_leak() {
            let a = Rc::new(Node(MyStruct { data: 5 }, RefCell::new(Rc::new(Nil))));

            println!("a initial rc count = {}", Rc::strong_count(&a));
            println!("a next item = {:?}", a.tail());

            let b = Rc::new(Node(MyStruct { data: 10 }, RefCell::new(Rc::clone(&a))));

            println!("a rc count after b creation = {}", Rc::strong_count(&a));
            println!("b initial rc count = {}", Rc::strong_count(&b));
            println!("b next item = {:?}", b.tail());

            // Comment to avoid cycle and memory leak
            if let Some(link) = a.tail() {
                *link.borrow_mut() = Rc::clone(&b);
            }

            println!("b rc count after changing a = {}", Rc::strong_count(&b));
            println!("a rc count after changing a = {}", Rc::strong_count(&a));
        }
    }

    pub mod weak {

        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }

        pub fn test_weak() {
            let leaf = Rc::new(Node {
                value: 3,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            });

            let leaf2 = Rc::new(Node {
                value: 4,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            });

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            branch.children.borrow_mut().push(Rc::clone(&leaf2));

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        }
    }

    pub mod arc {

        use std::sync::{Arc, Mutex};
        use std::thread;

        pub fn test_arc() {
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];

            for _ in 0..10 {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();

                    *num += 1;
                });
                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }

            println!("Result: {}", *counter.lock().unwrap());
        }
    }
}

fn main() {
    test_stack();
    println!("\n");
    test_list();
    println!("\n");
    test_tree();
    println!("\n");

    book::test_deref();
    println!("\n");
    book::test_drop();
    println!("\n");
    book::test_list();
    println!("\n");
    book::shared_list::test_shared_list();
    println!("\n");
    book::mut_shared_list::test_mut_shared_list();
    println!("\n");
    book::leak::test_leak();
    println!("\n");
    book::weak::test_weak();
    println!("\n");
    book::arc::test_arc();
    println!("\n");
}
