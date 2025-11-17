
use crate::dispatcher::Dispatcher;

/*
    -----------------------------------
        Orders for the products
    -----------------------------------
*/

pub struct Order {
    pub client: &String,
    pub products: &String,
    pub observer: &Dispatcher,
}

impl Order {
    /* 
        As in restaurants where they have a little bell to tell waiters 
        that an order is ready
    */

    fn ring_bell(&self) {
        self.observer.notify();
    }
}