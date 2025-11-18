
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

    println!("Zelda ordered a donut filled with strawberry jam and 
                pastry cream");
    orders.push(create_order("Zelda", "Food",
                 "Donut", &["Strawberry", "Pastry"], 
                 waiter));

    println!("Rodión ordered a vanilla muffin with chocolate chips");
    orders.push(create_order("Rodión", "Food",
                 "Muffin", &["Chocolate"], waiter));

    cafe.process_orders(&orders);
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
