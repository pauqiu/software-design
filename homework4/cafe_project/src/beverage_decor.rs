
use crate::beverages::Beverage;

/*
    -----------------------------
        Concrete decorators
    -----------------------------
*/

pub struct MilkDecorator {
    pub beverage: Box<dyn Beverage>
}

impl Beverage for MilkDecorator {
    fn prepare(&self) -> String {
        format!("{} with milk; ", self.beverage.prepare())
    }
}

pub struct SugarDecorator {
    pub beverage: Box<dyn Beverage>
}

impl Beverage for SugarDecorator {
    fn prepare(&self) -> String {
        format!("{} with sugar; ", self.beverage.prepare())
    }
}

pub struct CinnamonDecorator {
    pub beverage: Box<dyn Beverage>
}

impl Beverage for CinnamonDecorator {
    fn prepare(&self) -> String {
        format!("{} with cinnamon; ", self.beverage.prepare())
    }
}

pub struct VanillaDecorator {
    pub beverage: Box<dyn Beverage>
}

impl Beverage for VanillaDecorator {
    fn prepare(&self) -> String{
        format!("{} with vanilla; ", self.beverage.prepare())
    }
}

pub struct HoneyDecorator {
    pub beverage: Box<dyn Beverage>
}

impl Beverage for HoneyDecorator {
    fn prepare(&self) -> String {
        format!("{} with honey; ", self.beverage.prepare())
    }
}