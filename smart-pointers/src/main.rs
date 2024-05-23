use crate::List::{Nil, Cons};
use crate::CellList::{Nil as CellNil, Cons as CellCons};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::Deref;
use std::mem::drop;

// implementation of a the lisp con list data type(version of a linked list)
// DEPRECATED, this varriation causes the whole list to remain alive until all elements die
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

impl List {
    fn print(&self) {
        // Recursively moves through the list and prints each element
        match self {
            List::Cons(val, next) => {
                println!("{}", val);
                next.print(); // Recursive call
            },
            List::Nil => {}, // terminal case
        }
    }
}

// Liked list implentation for employing mutability contextually inside smart pointers
#[derive(Debug)]
enum CellList {
    Cons(Rc<RefCell<i32>>, Rc<CellList>),
    Nil,
}

// a node in a graph allowing for explicit child nodes(useful in building trees)
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// Using the deref trait with smart pointers
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
    
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Using the drop trait 
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}


fn main() {
    let list = Rc::new(Cons(3, Rc::new(Cons(2, Rc::new(Cons(1, Rc::new(Nil)))))) );
    list.print();

    // DEREF conventional demonstration
    let x = 5;
    let y = &x; // Normal reference(pointer to x)
    let z = Box::new(x); // Smart reference(smart pointer to x)
    let aleph = MyBox::new(x); // Home made smart pointer for deref(smart pointer to x)

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *aleph);

    // y, z are dereferenced by coercion but aleph has not had this functionality built in
    println!("5 = x = {} = y = {} = z = {} = aleph = {}", x, y, z, *aleph);

    // Here the custom smart pointer is just a wrapper around the string smart pointer
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created");
    drop(c);
    // drop called through the mem system to avoid the double free error!
    println!("CustomSmartPointer c dropped explicitly before end of main.");
    
    // Rc below!
    let _e = Cons(4, Rc::clone(&list));
    println!("count after creating e = {}", Rc::strong_count(&list));
    let f = Cons(5, Rc::clone(&list));
    println!("count after creating f = {}", Rc::strong_count(&list));
    f.print();
    {
        let _g = Cons(6, Rc::clone(&list));
        println!("Count after creating g = {}", Rc::strong_count(&list));
    }
    println!("count after g goes out of scope = {}", Rc::strong_count(&list));
    
    // Combining reference count with reference cell(for mutation)
    let value = Rc::new(RefCell::new(5));

    // note old a,b, and c are free to be dropped here. Shadowing is neat
    let a = Rc::new(CellCons(Rc::clone(&value), Rc::new(CellNil)));

    let b = CellCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = CellCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // Dereferenced from the Rc to get access to RefCell
    // RefCel morrowed as mutable where the inner value can then be changed
    *value.borrow_mut() += 10;

    // Notice that when these print b and c are sperate options for how the list grows
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leafe parent = {:?}", leaf.parent.borrow().upgrade());
    println!("Value = {}, leaf strong = {}, weak = {}",
        leaf.value,
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf child = {:?}", leaf.children);
    println!("Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // notice that d is dropped here implicitly
}
