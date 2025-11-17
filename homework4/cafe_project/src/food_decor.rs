
use crate::food::Food;

/*
    ------------------------------
            Base decorator
    ------------------------------
*/

trait FoodDecorator: Food {
    fn prepare(&self);
}

/*
    -----------------------------
        Concrete decorators
    -----------------------------
*/

struct ChocolateChipsDecorator<T: Food> {
    food: T
}

impl<T: Food> FoodDecorator for ChocolateChipsDecorator<T> {
    fn prepare(&self) {
        return self.food.prepare + " adding chocolate chips...";
    }
}

struct PeanutButterDecorator<T: Food> {
    food: T
}

impl<T: Food> FoodDecorator for PeanutButterDecorator<T> {
    fn prepare(&self) {
        return self.food.prepare + " adding peanut butter...";
    }
}

struct PastryCreamDecorator<T: Food> {
    food: T
}

impl<T: Food> FoodDecorator for PastryCreamDecorator<T> {
    fn prepare(&self) {
        return self.food.prepare + " adding pastry cream...";
    }
}

struct StrawberryJamDecorator<T: Food> {
    food: T
}

impl<T: Food> FoodDecorator for StrawberryJamDecorator<T> {
    fn prepare(&self) {
        return self.food.prepare + " adding strawberry jam...";
    }
}