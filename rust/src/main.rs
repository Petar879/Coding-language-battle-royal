use std::error::Error;
use std::io;
use serde_json::Value;
use rand::Rng;
use open;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    loop {
        println!("Enter option: ");
        let mut input_option = String::new();
        io::stdin().read_line(&mut input_option).expect("Failed to read line");
        let number: i32 = input_option.trim().parse().expect("Wanted a number");

        match number {
                1 => {
                    let resp = reqwest::get("https://api.senpy.club/v2/language/Rust").await?.text().await?;
                    let result: Value = serde_json::from_str(&resp).unwrap();
                    let rng = rand::thread_rng().gen_range(0..(result.as_array().expect("REASON").len() - 1));
                    let img_url = result[rng].as_str();
                    open::that(img_url.unwrap())?;
                },
                2 => {
                    //Select the programming language
                    let mut resp = reqwest::get("https://api.senpy.club/v2/languages").await?.text().await?;
                    let mut result: Value = serde_json::from_str(&resp).unwrap();
                    let mut rng = rand::thread_rng().gen_range(0..(result.as_array().expect("REASON").len() - 1));
                    let language_name = result[rng].as_str().unwrap().replace("#", "%23").replace("+", "%2B");

                    resp = reqwest::get(format!("https://api.senpy.club/v2/language/{}", language_name)).await?.text().await?;
                    result = serde_json::from_str(&resp).unwrap();
                    rng = rand::thread_rng().gen_range(0..(result.as_array().expect("REASON").len() - 1));
                    let img_url = result[rng].as_str();
                    open::that(img_url.unwrap())?;
                },
                _ => println!("Uknown option")
        }
    }
}