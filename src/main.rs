//41
use std::env;
use std::cell::RefCell;
#[macro_use]
extern crate std;

use std::collections::HashMap;



macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}
//53

//54

#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new()-> SimpleStack<T>{
        SimpleStack { 
            stack: RefCell::new(Vec::new()), 
        }
    }
    fn push(&self, value:T) {
        self.stack.borrow_mut().push(value);
    }
    fn pop(&self) -> Option<T>{
        self.stack.borrow_mut().pop()
    }
}
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    //41
    let map = hash_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3
    };

    println!("{:?}",map);
    //53
    let my_rc1 = MyRc::new(42);
    let my_rc2 = my_rc1.clone();

    println!("Strong Count 1: {}", my_rc1.strong_count());
    println!("Strong Count 2: {}", my_rc2.strong_count());

    // Access the inner resource through Deref trait.
    println!("Value: {}", *my_rc1);
    println!("Value: {}", *my_rc2);
    //54
    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());

    stack.push(4);
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
}

use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::NonNull;
use std::ops::Deref;

struct MyRc<T> {
    data: NonNull<T>,
    ref_count: AtomicUsize,
}

impl<T> MyRc<T> {
    pub fn new(data: T) -> Self {
        let data_ptr = Box::into_raw(Box::new(data));
        Self {
            data: NonNull::new(data_ptr).expect("Box::into_raw returned a null pointer"),
            ref_count: AtomicUsize::new(1),
        }
    }

    pub fn clone(&self) -> Self {
        let new_count = self.ref_count.fetch_add(1, Ordering::Relaxed) + 1;
        Self {
            data: self.data,
            ref_count: AtomicUsize::new(new_count),
        }
    }

    pub fn strong_count(&self) -> usize {
        self.ref_count.load(Ordering::Relaxed)
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let count = self.ref_count.fetch_sub(1, Ordering::Release);
        if count == 1 {
            // Last reference, we need to deallocate the data.
            unsafe {
                Box::from_raw(self.data.as_ptr());
            }
        }
    }
}