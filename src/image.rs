use crate::cataas;
use std::env::consts::OS;
pub fn render(){
    if cataas::get(".tmpcat.png"){
        let mut prev_size:(u16,u16)=(0,0);
        let mut conf:viuer::Config;
      loop{
          if &viuer::terminal_size() != &prev_size{
              conf = viuer::Config {
                  width: Some(viuer::terminal_size().0 as u32),
                  height: Some(viuer::terminal_size().1 as u32),
                  x: 0,
                  y: 0,
                  ..Default::default()
              };
              prev_size=viuer::terminal_size();
              clear_stdout();
              match viuer::print_from_file(".tmpcat.png", &conf){
                  Ok(_)=>{}
                  Err(_)=>{
                      print!("Either my code is broken, or your terminal is broken, so I'm blaming your terminal, because if my code was broken this shouldn't even be running")
                  }
              }
            }
        }
    /*clear_stdout();
      match std::fs::remove_file(".tmpcat.png"){
          Ok(_)=>{}
          Err(_)=>{}
      */}
}

fn clear_stdout(){
    if OS == "windows"{
        std::process::Command::new("cls").spawn().unwrap();
    }else{
        std::process::Command::new("clear").spawn().unwrap();
    }
}
