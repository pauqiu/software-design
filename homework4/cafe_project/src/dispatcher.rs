
/*
    -------------------------
            The Observer
    -------------------------
*/

struct Dispatcher;

impl Dispatcher {

    fn notify(&self, name: &String, product: &String) {
        self.announce_order(name, product);
    }

    fn announce_order(&self, name: &String, product: &String) {
        println!("I have a {} for {}", product, name);
    }
}