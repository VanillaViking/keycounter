use std::{error::Error, fs::{self, File}, io::Write};
use std::collections::HashMap;
use std::time::Instant;

pub struct Config {
    input_file_path: String,
    output_file_path: String,
    num_threads: u32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let input_file_path = match args.next() {
            Some(value) => value,
            None => return Err("no input file path provided"),
        };

        let output_file_path = match args.next() {
            Some(value) => value,
            None => return Err("no output file path provided"),
        };

        let num_threads: u32 = match args.next().unwrap_or(String::from("1")).parse() {
            Ok(value) => value,
            Err(_) => return Err("Unable to parse number of threads"),
        };

        Ok(Config {
            input_file_path,
            output_file_path,
            num_threads,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();
    let file_contents = fs::read_to_string(config.input_file_path)?;


    let mut keycount = HashMap::new();
    
    file_contents.lines().for_each(|md_line| count_keys(md_line, &mut keycount));

    let mut out_file = File::create(config.output_file_path)?;
    out_file.write("{\n".as_bytes())?;

    for keycode in keycount.keys() {

        let key = match keycode.as_str() {
            "24" => "q",
            "25" => "w",
            "26" => "e",
            "27" => "r",
            "28" => "t",
            "29" => "y",
            "30" => "u",
            "31" => "i",
            "32" => "o",
            "33" => "p",
            "34" => "{",
            "35" => "}",
            "38" => "a",
            "39" => "s",
            "40" => "d",
            "41" => "f",
            "42" => "g",
            "43" => "h",
            "44" => "j",
            "45" => "k",
            "46" => "l",
            "47" => ";",
            "48" => "'",
            "49" => "`",
            "52" => "z",
            "54" => "x",
            "56" => "c",
            "58" => "v",
            "60" => "b",
            "62" => "n",
            "64" => "m",
            "66" => ",",
            "68" => "?",
            default => {
                eprintln!("Unknown keycode {}", default);
                continue
            },
        };

        let count = keycount.get(keycode).unwrap_or(&0);
        out_file.write(format!("\"{key}\": \"{count}\",\n").as_bytes())?;
    }

    out_file.write("}".as_bytes())?;

    println!("done in: {}ms", now.elapsed().as_millis());

    Ok(())
}

pub fn count_keys(md_line: &str, keycount: &mut HashMap<String, u64>) {
    let count = keycount.entry(String::from(md_line.trim())).or_insert(0);
    *count += 1;
}



