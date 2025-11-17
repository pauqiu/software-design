/*
    ------------------------------
            Base beverage
    ------------------------------
*/

trait Beverage {
    fn prepare(&self);
}

/*
    -----------------------------
        Concrete beverages
    -----------------------------
*/

struct Coffee;

impl Beverage for Coffee {
    fn prepare(&self) {
        return "Brewing coffee...";
    }
}

struct Espresso;

impl Beverage for Espresso {
    fn prepare(&self) {
        return "Brewing espresso...";
    }
}

struct HotChocolate;

impl Beverage for HotChocolate {
    fn prepare(&self) {
        return "Brewing hot chocolate...";
    }
}

struct Latte;

impl Beverage for Latte {
    fn prepare(&self) {
        return "Brewing latte...";
    }
}

struct GreenTea;

impl Beverage for GreenTea {
    fn prepare(&self) {
        return "Brewing green tea...";
    }
}

struct ChamonilleTea;

impl Beverage for ChamonilleTea {
    fn prepare(&self) {
        return "Brewing chamonille tea...";
    }
}

struct Matcha;

impl Beverage for Matcha {
    fn prepare(&self) {
        return "Brewing matcha...";
    }
}