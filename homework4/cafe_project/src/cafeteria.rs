use crate::food::{Food, Toast, Croissant, Muffin, Donut};
use crate::beverages::{Beverage, Coffee, Espresso, HotChocolate, Latte, GreenTea};
use crate::food_decor::{
    ChocolateChipsDecorator, PastryCreamDecorator, PeanutButterDecorator, StrawberryJamDecorator
};
use crate::beverage_decor::{
    MilkDecorator, SugarDecorator, CinnamonDecorator, VanillaDecorator, HoneyDecorator
};
use crate::order::Order;

pub struct Cafeteria;

impl Cafeteria {
    pub fn process_orders(&self, orders: &[Order<'static>]) {

        for order in orders {
            if order.product_type == "Food" {
                self.cook(order);
            } else if order.product_type == "Beverage" {
                self.brew(order);
            }
        }

    }

    fn cook(&self, order: &Order) {
        
        let mut base_product: Box<dyn Food> = match order.base {
            "Toast" => Box::new(Toast),
            "Croissant" => Box::new(Croissant),
            "Muffin" => Box::new(Muffin),
            _ => Box::new(Donut),
        };

        for topping in order.toppings {
            base_product = self.add_toppings(base_product, topping);
        }

        let result = base_product.prepare();

        order.ring_bell(&order.client.to_string(), &result);
    }

    fn add_toppings(&self, base_product: Box<dyn Food>, topping: &str) -> Box<dyn Food> {
        match topping {
            "Chocolate" => Box::new(ChocolateChipsDecorator { food: base_product }),
            "Pastry" => Box::new(PastryCreamDecorator { food: base_product }),
            "Strawberry" => Box::new(StrawberryJamDecorator { food: base_product }),
            "Peanut" => Box::new(PeanutButterDecorator { food: base_product }),
            _ => base_product,
        }
    }

    fn brew(&self, order: &Order) {

        let mut base_beverage: Box<dyn Beverage> = match order.base {
            "Coffee" => Box::new(Coffee),
            "Latte" => Box::new(Latte),
            "Espresso" => Box::new(Espresso),
            "HotChocolate" => Box::new(HotChocolate),
            "GreenTea" => Box::new(GreenTea),
            _ => Box::new(Coffee), // fallback
        };

        for extra in order.toppings {
            base_beverage = self.add_beverage_extras(base_beverage, extra);
        }

        let result = base_beverage.prepare();

        order.ring_bell(&order.client.to_string(), &result);
    }

    fn add_beverage_extras(&self, base_beverage: Box<dyn Beverage>, extra: &str) -> Box<dyn Beverage> {

        match extra {
            "Milk" => Box::new(MilkDecorator { beverage: base_beverage }),
            "Sugar" => Box::new(SugarDecorator { beverage: base_beverage }),
            "Cinnamon" => Box::new(CinnamonDecorator { beverage: base_beverage }),
            "Vanilla" => Box::new(VanillaDecorator { beverage: base_beverage }),
            "Honey" => Box::new(HoneyDecorator { beverage: base_beverage }),
            _ => base_beverage,
        }
    }

}
