enum Light {
    Dull,
    Bright,
}

fn print_light_state(light: &Light) {
    match light {
        Light::Dull => println!("The light is currently Dull."),
        Light::Bright => println!("The light is currently Bright."),
    }
}

fn main() {
    let dull_light = Light::Dull;
    let bright_light = Light::Bright;

    print_light_state(&dull_light);
    print_light_state(&bright_light);
}