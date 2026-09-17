
fn is_grt50(x:i32){
    if x > 5{
        println!("Yes greater than 50");
    }else{
        println!("No it is less than 50");
    }
}


fn main() {
    let mut name="DUDE";
    println!("Hello, rust {name}");
    name="Hector";
    println!("Hello, rust {name}");
    is_grt50(40);
    let mut current_iter=0;
    while current_iter < 10 {
        if current_iter > 51 {
            println!("Done at {current_iter}");
            break;
        }
        current_iter+=1;
        println!("Current loop count:{current_iter}");
    }
}
