use reqwest::Client;
use scraper::{Html, Selector};
use std::env;
use std::process::Command;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let args: Vec<String> = env::args().collect();
    let res = client.get(&args[1]).send().await.unwrap();
    let body = res.text().await.unwrap();
    let document = Html::parse_document(&body);
    let a_sel = Selector::parse("h3 > a").unwrap();

    for sel in document.select(&a_sel) {
        let content = sel.text().collect::<Vec<_>>();
        let project = content[1].to_owned() + content[2];
        let p = format!("github.com/{}", remove_whitespace(&project));
        command(p);
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn command(project: String) {
    Command::new("google-chrome")
        .arg(project)
        .spawn()
        .expect("failed to open google-chrome");
}
