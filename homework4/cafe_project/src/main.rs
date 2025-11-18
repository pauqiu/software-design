
use crate::cafeteria::Cafeteria;
use crate::order::Order;
use crate::dispatcher::Dispatcher;

mod cafeteria;
mod order;
mod dispatcher;
mod food;
mod beverages;
mod food_decor;
mod beverage_decor;

fn main() {

    // The Observer
    let waiter = Dispatcher;

    // Where the products are assembled(wrapped)
    let cafe = Cafeteria;

    let mut orders = vec![]; 

    // Adding clients and orders
    println!("Paul ordered a toast with strawberry jam");
    orders.push(create_order("Paul", "Food",
                 "Toast", &["Strawberry"], waiter));

    println!("Zelda ordered a donut filled with strawberry jam and pastry cream");
    orders.push(create_order("Zelda", "Food",
                 "Donut", &["Strawberry", "Pastry"], 
                 waiter));

    println!("Rodión ordered a vanilla muffin with chocolate chips");
    orders.push(create_order("Rodión", "Food",
                 "Muffin", &["Chocolate"], waiter));

    println!("Annie ordered an espresso with vanilla extra");
    orders.push(create_order("Annie", "Beverage",
                 "Espresso", &["Vanilla"], waiter));

    println!("Bulma ordered a matcha with milk, honey and vanilla");
    orders.push(create_order("Bulma", "Beverage",
                 "Matcha", &["Milk", "Honey", "Vanilla"], 
                 waiter));

    println!("----------------------------");

    cafe.process_orders(&orders);

    /* 
        It always beats me how to coherently print of the components when
        implementing decorator. 
        And trying it while learning Rust wasn't that fun.
        At least it works :,)
    */

}

/*
    ----------------------------
        Auxiliar function
    ----------------------------
*/

fn create_order<'a>(client: &'a str, product_type: &'a str, base: &'a str, 
                    toppings: &'a [&'a str], observer:  Dispatcher) -> Order<'a> {
    Order {
        client,
        product_type,
        base,
        toppings,
        observer,
    }
}
