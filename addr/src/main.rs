enum Light {
    Dull,
    Bright,
}


struct Book<'a> {
    title: &'a str,
    desc: &'a str,
}


impl Light {
fn print_light_state(light: &Light) {
    match light {
        Light::Dull => println!("The light is currently Dull."),
        Light::Bright => println!("The light is currently Bright."),
    }
}
}


impl <'a>Book<'a>{
    fn print_title(&self){
        println!("{}",self.title);
    }
}

fn display_book(book: &Book){
    println!("Title: {}", book.title);
    println!("Desc: {}", book.desc);
}

fn main() {
    let dull_light = Light::Dull;
    let bright_light = Light::Bright;

    Light::print_light_state(&dull_light);
    Light::print_light_state(&bright_light);

    let book = Book {
        title: "This is my book",
        desc: "This is my bookeee",
    };

    book.print_title();
}
