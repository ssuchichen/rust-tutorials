trait Animal: AnimalEating + AnimalSound {}

trait AnimalEating {
    fn eat_food(&self);
}

trait AnimalSound {
    fn make_sound(&self);
}

struct Dog {}
impl AnimalEating for Dog {
    fn eat_food(&self) {
        println!("Dog is eating dog food");
    }
}
impl AnimalSound for Dog {
    fn make_sound(&self) {
        println!("Dog is barking");
    }
}
impl Animal for Dog {}

struct Antelope {}
impl AnimalEating for Antelope {
    fn eat_food(&self) {
        println!("Antelope is eating natural desert food");
    }
}
impl AnimalSound for Antelope {
    fn make_sound(&self) {
        println!("Antelope is bleating");
    }
}
impl Animal for Antelope {}

struct Bear {}
impl AnimalEating for Bear {
    fn eat_food(&self) {
        println!("Bear is eating some other animal")
    }
}
impl AnimalSound for Bear {
    fn make_sound(&self) {
        println!("Bear is roaring")
    }
}
impl Animal for Bear {}

// generic type
fn make_some_noise<T: Animal>(a: T) {
    a.make_sound();
}

// dynamic dispatch
fn make_some_noise2(a: &dyn Animal) {
    a.make_sound();
}

fn eat_some_food(a: &dyn Animal) {
    a.eat_food();
}

fn get_animal() -> Box<dyn Animal> {
    let bear = Bear {};
    Box::from(bear)
}

fn main() {
    let dog01 = Dog {};
    make_some_noise(dog01);

    let antelope01 = Antelope {};
    make_some_noise(antelope01);

    let dog02 = &Dog {};
    make_some_noise2(dog02);
    eat_some_food(dog02);

    let antelope02 = &Antelope {};
    make_some_noise2(antelope02);
    eat_some_food(antelope02);

    let animal = get_animal();
    animal.eat_food();
    animal.make_sound();
}
