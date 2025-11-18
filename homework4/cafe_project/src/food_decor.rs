
use crate::food::Food;

/*
    -----------------------------
        Concrete decorators
    -----------------------------
*/

pub struct ChocolateChipsDecorator {
    pub food: Box<dyn Food>,
}

impl Food for ChocolateChipsDecorator {
    fn prepare(&self) -> String {
        format!("{} with chocolate chips; ", self.food.prepare())
    }
}

pub struct PeanutButterDecorator {
    pub food: Box<dyn Food>
}

impl Food for PeanutButterDecorator {
    fn prepare(&self) -> String {
        format!("{} with peanut butter; ", self.food.prepare())
    }
}

pub struct PastryCreamDecorator {
    pub food: Box<dyn Food>
}

impl Food for PastryCreamDecorator {
    fn prepare(&self) -> String {
        format!("{} with pastry cream; ", self.food.prepare())
    }
}

pub struct StrawberryJamDecorator {
    pub food: Box<dyn Food>
}

impl Food for StrawberryJamDecorator {
    fn prepare(&self) -> String {
        format!("{} with strawberry jam; ", self.food.prepare())
    }
}