use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub mod app;
pub mod events;
pub mod generated;
pub mod geom;
pub mod widgets;

struct Component {
    pub state: String,
    pub parent: Option<Weak<RefCell<Component>>>,
    pub children: Vec<Rc<RefCell<Component>>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut parent = Rc::new(RefCell::new(Component {
            state: "parent".to_string(),
            parent: Default::default(),
            children: vec![],
        }));
        let child = Rc::new(RefCell::new(Component {
            state: "child".to_string(),
            parent: Some(Rc::downgrade(&parent)),
            children: vec![],
        }));
        parent.try_borrow_mut().unwrap().children.push(child.clone());

        let child = child.try_borrow_mut().unwrap();

        // code to dispatch an event
        let parent = child.parent.clone().unwrap(); // unwrap option
        let parent = parent.upgrade().unwrap(); // turn to strong ref
        let parent = parent.borrow_mut().state = "parent2".to_string(); // attempt mutable borrow
    }
}
