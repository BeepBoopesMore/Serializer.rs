use std::fs::{self, metadata, File, Metadata};
use std::env;
use std::net::Ipv4Addr;
use std::process::Command;
use std::thread::spawn;
use serde::{Deserialize,Serialize};
use serde_json::map::Values;
use serde_json::Value;
use std::io::prelude::*;
use std::io::BufReader;
use std::{thread,time};









// Sender for the send_data_to_server 
struct Sender{

}



//TODO
impl Sender{
    pub fn new() -> Self{
        let result = GetFileProfile::new().send_data_to_server();
        if !result{
            panic!("Just isnt the right choice obv")
        }
        else {
            Sender{}
        }

    }

}


fn clear_metadata_path(file_path:String){
    //TODO
}

struct GetFileProfile{
    path:String
}

impl GetFileProfile{
    fn new() -> Self{
        GetFileProfile{path:"profile.json".to_string()}
    }

    fn time_file(&mut self) -> String{
        let file = File::open("profile.json").expect("Error");
        let reader = BufReader::new(file);
        let u:serde_json::Value = serde_json::from_reader(reader).unwrap();
        u["time_file"].to_string()
    }
    fn send_data_to_server(&mut self) -> bool {
        let file = File::open(&self.path).expect("Error");
        let reader = BufReader::new(file);
        let u:serde_json::Value = serde_json::from_reader(reader).unwrap();
        let value:bool = match u["send_data"].as_str(){
            Some("false") => false,
            Some("False") => false,
            Some("True") => true,
            Some("true") => true,
            _ => false,
        };

         value
    }   
    
}


struct ProcessOpener{
    file_path : String

}

pub fn operation_mac_os(file_path:&str){
    //TODO
    // Open the app , load the profile settings and check what the time they want to see the file before closing and deleting is 
    // Check if they want to send the file to a server before ,  to change the metadata before so
    // First open the app 

    let handle = thread::spawn(move ||
    {
        //TODO
    });

}


pub fn operation_unix(){
    //TODO
}


pub fn operation_windows(){
    //TODO
}
impl ProcessOpener{
    fn new(file_path:String) -> Self{
        ProcessOpener {file_path}
    }

    fn open_task(&mut self){
        let env = env::consts::OS;
        match env{


            "macos" => {
                operation_mac_os(&self.file_path);
            }

            "windows" => {
                operation_windows();
            }

            "unix" => {
                operation_unix();
            }

            _ => println!("Error this has not been made for ur OS!")
        }
    }

}


struct FileMetadata{
    file_path : String,
    metadata : Option<String>

}




impl FileMetadata{
    fn new(file_path:String) -> Self{
       FileMetadata{file_path,metadata:None}   


    }

    fn file_lenght(&mut self ) -> std::io::Result<Metadata>{
        let metadata = fs::metadata(&self.file_path)?;
        Ok(metadata)
    }





}



fn main() -> std::io::Result<()>{
    //let time = GetFileProfile::new().time_file();
    let time = GetFileProfile::new().send_data_to_server();
    println!("{}",time);
    let send_data = GetFileProfile::new().send_data_to_server();



   for i in 0..=1{
    println!("Hi");
   }
   Ok(())








    
}