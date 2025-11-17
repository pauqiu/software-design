
use crate::beverages::Beverage;

/*
    ------------------------------
            Base decorator
    ------------------------------
*/

trait BeverageDecorator: Beverage {
    fn prepare(&self);
}

/*
    -----------------------------
        Concrete decorators
    -----------------------------
*/

struct MilkDecorator<T: Beverage> {
    beverage: T
}

impl<T: Beverage> BeverageDecorator for MilkDecorator<T> {
    fn prepare(&self) {
        return self.beverage.prepare + " adding milk...";
    }
}

struct SugarDecorator<T: Beverage> {
    beverage: T
}

impl<T: Beverage> BeverageDecorator for SugarDecorator<T> {
    fn prepare(&self) {
        return self.beverage.prepare + " adding sugar...";
    }
}

struct CinnamonDecorator<T: Beverage> {
    beverage: T
}

impl<T: Beverage> BeverageDecorator for CinnamonDecorator<T> {
    fn prepare(&self) {
        return self.beverage.prepare + " adding cinnamon...";
    }
}

struct VanillaDecorator<T: Beverage> {
    beverage: T
}

impl<T: Beverage> BeverageDecorator for VanillaDecorator<T> {
    fn prepare(&self) {
        return self.beverage.prepare + " adding vanilla...";
    }
}

struct HoneyDecorator<T: Beverage> {
    beverage: T
}

impl<T: Beverage> BeverageDecorator for HoneyDecorator<T> {
    fn prepare(&self) {
        return self.beverage.prepare + " adding honey...";
    }
}