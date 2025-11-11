use std::{any::Any, fmt::Debug};

trait Animal: Debug {
    fn speak(&self);
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

#[derive(Debug)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

fn speak_twice(animal: &impl Animal) {
    println!("{animal:?}");
    animal.speak();
    animal.speak();
}

fn make_animal() -> impl Animal {
    Cat
}

trait DowncastableAnimal {
    fn speak(&self) {
        println!("Default speak sound!")
    }
    fn as_any(&self) -> &dyn Any;
}

struct Tortoise;

impl DowncastableAnimal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let cat = Cat;
    cat.speak();
    let dog = Dog;
    dog.speak();
    speak_twice(&cat);

    let animal = make_animal();
    speak_twice(&animal);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Cat), Box::new(Dog)];
    animals.iter().for_each(|animal| animal.speak());

    let more_animals: Vec<Box<dyn DowncastableAnimal>> = vec![Box::new(Tortoise)];
    for animal in more_animals.iter() {
        if let Some(t) = animal.as_any().downcast_ref::<Tortoise>() {
            t.speak();
            println!("I'm a tortoise?!");
        }
    }
}
