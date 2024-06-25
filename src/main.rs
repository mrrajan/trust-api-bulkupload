use std::fs;
use reqwest::Url;
use reqwest::blocking::Client;
use rand::prelude::*;
use serde_json::Value;
use log::{info, error};
use simplelog::*;
use rand::distributions::Alphanumeric;

fn upload_sbom(path: &str, baseurl: &str, endpoint: &str, token: &str){
    let file_path = fs::read_dir(path).unwrap();
    for file in file_path{
        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(0..1000); 
        let rand_string: String = thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(4)
                                .map(char::from)
                                .collect();
        info!("uploading file... {:?}", &file);
        let str_file_content:String = fs::read_to_string(&file.unwrap().path()).expect("Fail to read file");
        let json_content: serde_json::Value = serde_json::from_str::<serde_json::Value>(&str_file_content).unwrap();
        let id = "test-bulk-".to_owned() + &rand_string + "-" + &random_number.to_string();
        let request_url_str = baseurl.to_string() + endpoint + "?id=" + &id;
        let req_url: Url = Url::parse(&request_url_str).expect("Error parsing");
        let client = Client::builder().danger_accept_invalid_certs(true).build().expect("Failed");
        let response = client.post(req_url).header("Authorization",format!("Bearer {}", token)).json(&json_content).send().expect("fail");
        info!("Response for the id {} is {}", id, response.status());
    }
    
}

fn main(){
    CombinedLogger::init(
        vec![

            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed,ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), std::fs::File::create("bulkupload.log").unwrap()),
        ]
    ).unwrap();

    let path = "";
    let base_url="";
    let endpoint = "/api/v1/sbom";
    let token = "";
    upload_sbom(path, base_url, endpoint, token);
    
}