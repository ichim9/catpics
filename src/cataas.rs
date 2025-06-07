use reqwest::blocking;
pub fn get(target_file:&str) -> bool{
    match blocking::get("https://cataas.com/cat"){
        Ok(k)=>{
            match k.bytes(){
                Ok(k)=>{
                    match std::fs::write(target_file,k){
                        Ok(_)=>{true}
                        Err(e)=>{
                            println!("Error writing file");
                            println!("Error will be displayed below:\n\n{e}\n\n");
                            false
                        }
                    }
                }
                Err(e)=>{
                    println!("Error converting to bytes");
                    println!("Error will be displayed below:\n\n{e}\n\n");
                    false
                }
            }
        }
        Err(e)=>{
            println!("Error connecting to catAAS?\n is your wifi broken?");
            println!("Error will be displayed below:\n\n{e}\n\n");
            false
        }
    }
}
