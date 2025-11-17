
/*
    ------------------------------
            Base food
    ------------------------------
*/

trait Food {
    fn prepare(&self);
}

/*
    -----------------------------
        Concrete food
    -----------------------------
*/

struct Toast;

impl Food for Toast {
    fn prepare(&self) {
        return "Making toast...";
    }
}

struct Croissant;

impl Food for Croissant {
    fn prepare(&self) {
        return "Making croissant...";
    }
}

struct Muffin;

impl Food for Muffin {
    fn prepare(&self) {
        return "Making muffin...";
    }
}

struct Donut;

impl Food for Donut {
    fn prepare(&self) {
        return "Making donut...";
    }
}