extern crate clap;
extern crate reqwest;
extern crate viuer;
mod cataas;
mod image;

use clap::Command;
fn main(){
    let cmd = shell().get_matches();
    match cmd.subcommand(){
        Some(("get",_))=>{cataas::get("catpic.png");}
        Some(("view",_))=>{image::render()}
        _=>{}
    }
}

fn shell() -> Command{
    Command::new("catpics").about("I wonder what the name of this implies, I personally haven't a clue!").subcommand(
        Command::new("get").about("Fetch a cat pic and download it as 'catpic.png'")
    )
    .subcommand(
        Command::new("view").about("View a cat pic, this stores the catpic temporarily as .tmpcat.png")
    ).subcommand_required(true)
}
