enum Light {
    Dull,
    Bright,
}


struct Book{
    title:&str;
    desc:&str;
}

fn print_light_state(light: &Light) {
    match light {
        Light::Dull => println!("The light is currently Dull."),
        Light::Bright => println!("The light is currently Bright."),
    }
}

fn display_book(book:Book){
    println!("Title:{book.title}");
    println!("Title:{book.title}");
}

fn main() {
    let dull_light = Light::Dull;
    let bright_light = Light::Bright;

    print_light_state(&dull_light);
    print_light_state(&bright_light);

    let book={
        title:"This is my book",
        desc:"This is my book"
    };

    display_book(book)

}
