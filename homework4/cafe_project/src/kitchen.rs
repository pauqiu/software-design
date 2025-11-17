
use crate::food::{Toast, Croissant, Muffin, Donut};
use crate::beverages::{Coffee, Espresso, HotChocolate, Latte, GreenTea, ChamonilleTea};

struct Cafeteria;

impl Cafeteria {
    fn process_orders(&self, orders: &[&String]) {

        for order in orders {
            if order.contains("Food") {
                self.cook(order);
            } else {
                self.barista(order);
            }
        }

    }

    fn cook(&self, order: &String) {

        if order.contains("Toast") {

        }
    }

    fn barista(&self, order: &String) {

    }
}