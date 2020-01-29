/// Boxed Two-Way Linked list
#[derive(Clone, Debug, Eq, PartialEq)]
struct Btwl {
    pub pre: Option<*mut Box<Btwl>>,
    pub next: Option<Box<Btwl>>,
}

impl Btwl {
    pub fn new() -> Btwl {
        Btwl {
            pre: None,
            next: None,
        }
    }

    pub fn test() {
        let mut r = Box::new(Btwl::new());
        let mut c = Box::new(Btwl::new());

        c.pre = Some(&mut r);
        r.next = Some(c.clone());

        assert_eq!(r.next, Some(c));
    }
}

/// Rc Two-Way Linked List
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;
#[derive(Debug, Eq, PartialEq)]
struct Rtwl {
    pub pre: *mut Option<Rc<RefCell<Rtwl>>>,
    pub next: Option<Rc<RefCell<Rtwl>>>,
}

impl Rtwl {
    pub fn new() -> Rtwl {
        Rtwl {
            pre: ptr::null_mut(),
            next: None,
        }
    }

    pub fn test() {
        let r = Rc::new(RefCell::new(Rtwl::new()));
        let c = Rc::new(RefCell::new(Rtwl::new()));
        c.borrow_mut().pre = &mut Some(r.clone());
        r.borrow_mut().next = Some(c.clone());

        println!("c: {:#?}", &c.borrow().pre);
        assert_eq!(r.borrow().next, Some(c));
    }
}

fn main() {
    Btwl::test();
    Rtwl::test();
}
