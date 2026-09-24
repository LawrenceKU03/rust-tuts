enum Matches{
    Chelsea,
    ManUnited,
    France
}


struct Car{
    model:String,
    engine:String,
    price:i32
}

fn main() {
    println!("Hello, world!");

    let match_=Matches::Chelsea;
    
    match match_ {
        Matches::Chelsea => println!("Chelsea"),
        Matches::ManUnited => println!("ManUnited"),
        Matches::France => println!("France"),
    };

    
  let lambo=Car {
      model:"Lamb".to_string(),
      engine:"V8".to_string(),
      price:500_000
  };

  let (x,y)=(10,20);

  println!("{}",lambo.model);
  println!("{}",x);


}
