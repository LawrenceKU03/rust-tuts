enum Light {
    Dull,
    Bright,
}


struct Book<'a> {
    title: &'a str,
    desc: &'a str,
}

fn print_light_state(light: &Light) {
    match light {
        Light::Dull => println!("The light is currently Dull."),
        Light::Bright => println!("The light is currently Bright."),
    }
}

fn display_book(book: &Book){
    println!("Title: {}", book.title);
    println!("Desc: {}", book.desc);
}

fn main() {
    let dull_light = Light::Dull;
    let bright_light = Light::Bright;

    print_light_state(&dull_light);
    print_light_state(&bright_light);

    let book = Book {
        title: "This is my book",
        desc: "This is my book",
    };

    display_book(&book);
 display_book(&book);
}
