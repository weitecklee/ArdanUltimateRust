struct Cat(String);

impl Cat {
    fn feed(&mut self) {
        println!("{} nom nom.", self.0);
        self.0 = format!("{} (purring)", self.0);
    }
}

struct CatFeeder<'a> {
    cat: &'a mut Cat,
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut cats = [Cat("Foosball".to_string()), Cat("Poofball".to_string())];

    let mut feeders = Vec::new();

    for cat in cats.iter_mut() {
        feeders.push(CatFeeder { cat });
    }

    feeders.iter_mut().for_each(|f| f.feed());
}
