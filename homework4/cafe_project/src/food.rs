
/*
    ------------------------------
            Base food
    ------------------------------
*/

pub trait Food {
    fn prepare(&self) -> String;
}

/*
    -----------------------------
        Concrete food
    -----------------------------
*/

pub struct Toast;

impl Food for Toast {
    fn prepare(&self) -> String {
        return "Toast".to_string();
    }
}

pub struct Croissant;

impl Food for Croissant {
    fn prepare(&self) -> String {
        return "Croissant".to_string();
    }
}

pub struct Muffin;

impl Food for Muffin {
    fn prepare(&self) -> String {
        return "Muffin".to_string();
    }
}

pub struct Donut;

impl Food for Donut {
    fn prepare(&self) -> String {
        return "Donut".to_string();
    }
}