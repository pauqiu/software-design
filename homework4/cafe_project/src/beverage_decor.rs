
use crate::beverages::Beverage;

/*
    -----------------------------
        Concrete decorators
    -----------------------------
*/

pub struct MilkDecorator<T: Beverage> {
    pub beverage: T
}

impl<T: Beverage> Beverage for MilkDecorator<T> {
    fn prepare(&self) -> String {
        format!("{} with milk; ", self.beverage.prepare())
    }
}

pub struct SugarDecorator<T: Beverage> {
    pub beverage: T
}

impl<T: Beverage> Beverage for SugarDecorator<T> {
    fn prepare(&self) -> String {
        format!("{} with sugar; ", self.beverage.prepare())
    }
}

pub struct CinnamonDecorator<T: Beverage> {
    beverage: T
}

impl<T: Beverage> Beverage for CinnamonDecorator<T> {
    fn prepare(&self) -> String {
        format!("{} with cinnamon; ", self.beverage.prepare())
    }
}

pub struct VanillaDecorator<T: Beverage> {
    beverage: T
}

impl<T: Beverage> Beverage for VanillaDecorator<T> {
    fn prepare(&self) -> String{
        format!("{} with vanilla; ", self.beverage.prepare())
    }
}

pub struct HoneyDecorator<T: Beverage> {
    beverage: T
}

impl<T: Beverage> Beverage for HoneyDecorator<T> {
    fn prepare(&self) -> String {
        format!("{} with honey; ", self.beverage.prepare())
    }
}