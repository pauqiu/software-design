
use crate::food::{Food, Toast, Croissant, Muffin, Donut};
use crate::beverages::{Beverage, Coffee, Espresso, HotChocolate, Latte, 
                        GreenTea};
use crate::food_decor::{ChocolateChipsDecorator, PastryCreamDecorator, 
                        PeanutButterDecorator, StrawberryJamDecorator};
use crate::beverage_decor::{MilkDecorator, SugarDecorator, CinnamonDecorator, 
                            VanillaDecorator, HoneyDecorator};
use crate::order::Order;

pub struct Cafeteria;

impl Cafeteria {
    pub fn process_orders(&self, orders: &[Order<'static>]) {

        for order in orders {
            if order.product_type == "Food" {
                self.cook(order);
            } 
        }

    }

    fn cook(&self, order: &Order) {

        let base_product: Box<dyn Food> = match order.base {
            "Toast" => Box::new(Toast),
            "Croissant" => Box::new(Croissant),
            "Muffin" => Box::new(Muffin),
            _ => Box::new(Donut),
        };

        self.add_toppings(base_product, );

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

}