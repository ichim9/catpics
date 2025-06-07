use crate::cataas;
use std::env::consts::OS;
pub fn render(){
    if cataas::get(".tmpcat.png"){
      let conf = viuer::Config {
          width: Some(120),
          height: Some(50),
          x: 30,
          y: 0,
          ..Default::default()
      };
      if OS == "windows"{
          std::process::Command::new("cls").spawn().unwrap();
      }else{
          std::process::Command::new("clear").spawn().unwrap();
      }
      match viuer::print_from_file(".tmpcat.png", &conf){
          Ok(_)=>{}
          Err(_)=>{
              print!("Either my code is broken, or your terminal is broken, so I'm blaming your terminal, because if my code was broken this shouldn't even be running")
          }
      }
    match std::fs::remove_file(".tmpcat.png"){
        Ok(_)=>{}
        Err(_)=>{}
    }
    let mut useless_input = String::new();
    std::io::stdin().read_line(&mut useless_input).unwrap();
    if OS == "windows"{
        std::process::Command::new("cls").spawn().unwrap();
    }else{
        std::process::Command::new("clear").spawn().unwrap();
    }
}else{
}
}
