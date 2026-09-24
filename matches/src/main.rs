enum Matches{
    Chelsea,
    ManUnited,
    France
}

fn main() {
    println!("Hello, world!");

    let match_=Matches::Chelsea;
    
    match match_ {
        Matches::Chelsea => println!("Chelsea"),
        Matches::ManUnited => println!("ManUnited"),
        Matches::France => println!("France"),
    }



}
