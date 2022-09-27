use reqwest::Url;
use scraper::Selector;
use std::collections::HashMap;
use std::env;
use std::io::{BufWriter, ErrorKind};
use text_colorizer::*;

#[derive(Debug, PartialEq)]
enum DownloadType {
    Page,
    Courses,
}

#[derive(Debug)]
struct Arguments {
    url: String,
    task: DownloadType,
    out_folder: String,
}

#[derive(Debug)]
enum ArgsParsingError {
    IncorrectLen,
    UrlParsing,
}

fn get_word_list(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<HashMap<String, reqwest::Url>, Box<dyn std::error::Error>> {
    const WORD_LIST_SELECTOR: &str = ".sc-iFMAIt";
    const WORD_SELECTOR: &str = "strong";
    const URL_SELECTOR: &str = r#"source[type="audio/MP3"]"#;

    let selector = Selector::parse(WORD_LIST_SELECTOR).unwrap();
    let name_selector = Selector::parse(WORD_SELECTOR).unwrap();
    let url_selector = Selector::parse(URL_SELECTOR).unwrap();

    let response = client.get(url.to_owned()).send()?.text()?;
    let html = scraper::Html::parse_document(&response);
    let input_it = html.select(&selector);

    let mut word_list: HashMap<String, reqwest::Url> = HashMap::new();

    for section in input_it {
        let mut word = section
            .select(&name_selector)
            .next()
            .unwrap()
            .inner_html()
            .replace("etwas", "")
            .replace("jemanden", "")
            .replace('?', " (Frage)")
            .replace('!', " (Ausrufe)")
            .replace('|', "")
            .replace('.', "")
            .replace("/ ", "");

        if let Some(pos_first) = word.find('(') {
            if let Some(pos_last) = word.find(')') {
                word.drain(pos_first..=pos_last);
            }
        }

        word = word.replace('/', "-");

        if word.is_empty() {
            continue;
        }

        if word.find('/').is_some() {
            println!(
                "Skipping word section: {} as character '/' was found.",
                word.red(),
            );
            continue;
        }

        word = word.trim().to_string();

        let url = Url::parse(
            section
                .select(&url_selector)
                .next()
                .unwrap()
                .value()
                .attr("src")
                .unwrap(),
        )?;

        word_list.insert(word, url);
    }
    Ok(word_list)
}

fn parse_course_list(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    const COURSE_LIST_SELECTOR: &str = ".sc-cNKqjZ";

    let selector = Selector::parse(COURSE_LIST_SELECTOR).unwrap();

    let response = client.get(url.to_owned()).send()?.text()?;
    let html = scraper::Html::parse_document(&response);
    let input_it = html.select(&selector);

    let mut url_list: Vec<String> = Vec::new();

    for section in input_it {
        let url = section.value().attr("href").unwrap();

        println!("Url: {}", url);

        url_list.push(url.to_string());
    }

    create_abs_urls(url, &url_list)
}

fn create_abs_urls(
    base_url: &str,
    url_list: &Vec<String>,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut abs_url_list = Vec::new();
    for rel_url in url_list {
        let abs_url = Url::parse(base_url)?
            .join((rel_url.to_string() + "/").as_str())?
            .join("lv")?;
        abs_url_list.push(abs_url.to_string());
    }
    Ok(abs_url_list)
}

fn download_and_save(
    client: &reqwest::blocking::Client,
    folder: &str,
    name: &str,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(folder)?;

    let file = match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(folder.to_owned() + "/" + name)
    {
        Ok(val) => val,
        Err(err) if err.kind() == ErrorKind::AlreadyExists => {
            println!("File: {} already exists", name);
            return Ok(());
        }
        Err(err) => return Err(Box::new(err)),
    };

    let mut writer = BufWriter::new(file);

    match client.get(url.to_owned()).send()?.copy_to(&mut writer) {
        Ok(_) => {
            println!("Saved file: {}", name.bold());
            Ok(())
        }
        Err(err) => Err(err)?,
    }
}

fn process_args(args: env::Args) -> Result<Arguments, ArgsParsingError> {
    // 3 Args for now. Program name, flag, url, output_folder
    let args: Vec<String> = args.skip(1).collect();
    if args.len() != 3 {
        eprintln!(
            "{} wrong number of arguments: expected 3, got {}",
            "Error:".red().bold(),
            args.len()
        );
        return Err(ArgsParsingError::IncorrectLen);
    }

    let arguments = Arguments {
        url: Url::parse(&args[0])
            .map(|url| url.to_string())
            .map_err(|_err| ArgsParsingError::UrlParsing)?,
        task: if args[1] == "c" {
            DownloadType::Courses
        } else {
            DownloadType::Page
        },
        out_folder: args[2].clone(),
    };

    Ok(arguments)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = match process_args(env::args()) {
        Ok(args) => args,
        Err(err) => {
            eprintln!("Error processing arguments. Error: {:?}", err);
            std::process::exit(1);
        }
    };

    let client = reqwest::blocking::Client::builder().build()?;

    let mut url_list = vec![arguments.url.to_owned()];
    if arguments.task == DownloadType::Courses {
        url_list = parse_course_list(&client, &arguments.url)?;
    }
    for url in url_list {
        let word_list = get_word_list(&client, url.as_str())?;
        for (name, url) in word_list {
            println!("Word: {}", name);
            download_and_save(&client, &arguments.out_folder, &name, url.as_str())?;
        }
    }

    Ok(())
}
