/*
    ------------------------------
            Base beverage
    ------------------------------
*/

pub trait Beverage {
    fn prepare(&self) -> String;
}

/*
    -----------------------------
        Concrete beverages
    -----------------------------
*/

pub struct Coffee;

impl Beverage for Coffee {
    fn prepare(&self) -> String {
        return "Coffee".to_string();
    }
}

pub struct Espresso;

impl Beverage for Espresso {
    fn prepare(&self) -> String {
        return "Espresso".to_string();
    }
}

pub struct HotChocolate;

impl Beverage for HotChocolate {
    fn prepare(&self) -> String {
        return "Hot chocolate".to_string();
    }
}

pub struct Latte;

impl Beverage for Latte {
    fn prepare(&self) -> String {
        return "Latte".to_string();
    }
}

pub struct GreenTea;

impl Beverage for GreenTea {
    fn prepare(&self) -> String {
        return "Green tea".to_string();
    }
}

pub struct Matcha;

impl Beverage for Matcha {
    fn prepare(&self) -> String {
        return "Matcha".to_string();
    }
}