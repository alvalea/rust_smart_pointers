use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result};
use std::option::Option;
use std::rc::{Rc, Weak};

mod book;

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

impl<T: Debug> Debug for StackNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("StackNode")
            .field("value", &self.value)
            .field("next", &self.next)
            .finish()
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
            println!("{:#?}", current);
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

impl<T: Debug> Debug for ListNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("ListNode")
            .field("value", &self.value)
            .field("next", &self.next)
            .finish()
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
            println!("{:#?} -> ", current);
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
        false
    }

    fn print(node: &Rc<RefCell<Self>>) {
        let current_node = node.borrow();
        println!("{:#?} ", current_node);
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

impl<T: Debug> Debug for TreeNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("TreeNode")
            .field("value", &self.value)
            .field("parent", &self.parent)
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_demo() {
        test_stack();
    }

    #[test]
    fn list_demo() {
        test_list();
    }

    #[test]
    fn tree_demo() {
        test_tree();
    }

    #[test]
    fn book_deref_demo() {
        book::test_deref();
    }

    #[test]
    fn book_drop_demo() {
        book::test_drop();
    }

    #[test]
    fn book_list_demo() {
        book::test_list();
    }

    #[test]
    fn book_shared_list_demo() {
        book::shared_list::test_shared_list();
    }

    #[test]
    fn book_mut_shared_list_demo() {
        book::mut_shared_list::test_mut_shared_list();
    }

    #[test]
    fn book_leak_demo() {
        book::leak::test_leak();
    }

    #[test]
    fn book_weak_demo() {
        book::weak::test_weak();
    }

    #[test]
    fn book_arc_demo() {
        book::arc::test_arc();
    }
}
