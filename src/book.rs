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

use self::List::{Nil, Node};

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

    use self::List::{Nil, Node};
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

    use self::List::{Nil, Node};
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

    use self::List::{Nil, Node};
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
