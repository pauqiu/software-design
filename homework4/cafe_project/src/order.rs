
use crate::dispatcher::Dispatcher;

/*
    -----------------------------------
        Orders for the products
    -----------------------------------
*/

pub struct Order<'a> {
    pub client: &'a str,
    pub product_type: &'a str,
    pub base: &'a str,
    pub toppings: &'a [&'a str],
    pub observer: Dispatcher,
}

impl<'a> Order<'a> {
    /* 
        As in restaurants where they have a little bell to tell waiters 
        that an order is ready
    */

    pub fn ring_bell(&self, name: &String, product: &String) {
        self.observer.notify(name, product);
    }
}