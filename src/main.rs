extern crate clap;
extern crate reqwest;
use clap::Command;
use reqwest::blocking::get;
fn main() {
    let cmd = shell().get_matches();
    match cmd.subcommand(){
        Some(("get",_))=>{
            match get("https://cataas.com/cat"){
                Ok(k)=>{
                    match k.bytes(){
                        Ok(k)=>{
                            match std::fs::write("catpic.png",k){
                                Ok(_)=>{}
                                Err(e)=>{
                                    println!("Error writing file");
                                    println!("Error will be displayed below:\n\n{e}\n\n");
                                }
                            }
                        }
                        Err(e)=>{
                            println!("Error converting to bytes");
                            println!("Error will be displayed below:\n\n{e}\n\n");
                        }
                    }
                }
                Err(e)=>{
                    println!("Error connecting to catAAS?\n is your wifi broken?");
                    println!("Error will be displayed below:\n\n{e}\n\n");
                }
            }
        }
        _=>{}
    }
}

fn shell() -> Command{
    Command::new("catpics").about("I wonder what the name of this implies, I personally haven't a clue!").subcommand(
        Command::new("get").about("Fetch a cat pic and download it as 'catpic.png'")
    ).subcommand_required(true)
}
