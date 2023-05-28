use dotenv;
use clap::{arg, Parser};
use reqwest;

const LAT: f32 = -41.2;
const LON: f32 = 174.7;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    /// Number of days for the forecast.
    #[arg(short, default_value_t = 0)]
    days: u8,
}

fn main() -> Result<(), Box<reqwest::Error>>{

    dotenv::dotenv().unwrap();

    let mut api_key: Option<String> = None;
    for  (key, value)  in std::env::vars() {
        if key != "APIKEY" {
            continue;
        }
        api_key = Some(value)
    }

    if api_key.is_none(){
        panic!("Need API Key");
    }

    let api_key: String = api_key.unwrap();

    let args = Args::parse();

    let method: &str = match args.days {
        0 => "weather",
        _ => "forecast",
    };
    let cnt: u8 = args.days * 8;

    let url: String = format!("https://api.openweathermap.org/data/2.5/{method}?lat={LAT}&lon={LON}&appid={api_key}&units=metric&cnt={cnt}");

    println!("{}", url);

    let body: String = reqwest::blocking::get(url)?.text()?;

    println!("body = {:?}", body);

    Ok(())
}


//16:04