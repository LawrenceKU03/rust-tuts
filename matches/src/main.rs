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
      model:"Lambo".to_string(),
      engine:"V8".to_string(),
      price:500_000
  };

  let (x,y)=(10,20);

  let is_lambo={
      if lambo.model.to_lowercase() == "lambo".to_string().to_lowercase(){
          true
      }else{
          false
      }
  };
  println!("{}",lambo.model);
  println!("{}",x);
  println!("{}",is_lambo);


}
