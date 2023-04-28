
use std::process::exit;

use scraper::Html;

use crate::mtools::{exec::run_command, parse::parser, scrape, downloader::download_package};

mod mtools;

static LINUX_FROM_SCRATCH: &str = "https://www.linuxfromscratch.org/blfs/view/9.1/";
static HELP: &str = "
usage:
for install: minstall -i or --install <package_name>
for search package: minstall -s  or --search <package_name>
";

async fn search_package(package: String) -> Result<Vec<(String, String)>, reqwest::Error> {
    let request = format!("{LINUX_FROM_SCRATCH}index.html");
    match reqwest::get(request).await {
        Ok(response) => {
            let r = response.text().await.unwrap();
            let html_page = Html::parse_document(&r);

            let package_name: Vec<String> =
                scrape::get_element_inner_html(&html_page, "li.sect1>a");

            let link_to_package: Vec<String> =
                scrape::get_element_attribute(&html_page, "li.sect1>a", "href");

            Ok(package_name
                .into_iter()
                .zip(link_to_package)
                .filter(|(name, _link)| {
                    let package_name = name.split("-").collect::<Vec<&str>>()[0].to_string();
                    package_name.to_lowercase() == package.to_lowercase()
                })
                .collect())
        }
        Err(error) => Err(error),
    }
}

async fn install_the_package(link: &str) -> Result<(), reqwest::Error> {
    let link = format!("{LINUX_FROM_SCRATCH}{link}");
    match reqwest::get(link).await {
        Ok(response) => {
            let r = response.text().await.unwrap();
            let html_page = Html::parse_document(&r);

            let url: &String = &scrape::get_element_attribute::<Vec<String>>(
                &html_page,
                r#"a[class="ulink"]"#,
                "href",
            )[0];

            let build_command: Vec<String> =
                scrape::get_element_inner_html(&html_page, r#"kbd[class="command"]"#);

            if build_command.len() >= 2 {
                let build = &build_command[0]
                    .split("&amp;&amp;\n")
                    .map(|c| c.to_string().replace("\n", ""))
                    .collect::<Vec<String>>();
                let install = &build_command[1];

                println!("try to download: {url}");
                download_package(url).expect("download failed"); 

                // etap build
                for cmd in build {
                    if let Err(_) = run_command(&cmd) {
                        println!("build failed");
                        exit(1);
                    }
                }
                // install the package
                if let Err(_) = run_command(&install) {
                    println!("installation failed");
                    exit(1);
                }
            }
            Ok(())
        }
        Err(error) => Err(error),
    }
}

async fn cli() {
    match parser() {
        Ok(output) => {
            let (args, package) = output;
            match args.as_str() {
                "-i" | "--install" => match search_package(package).await {
                    Ok(value) => install_the_package(&value[0].1).await.unwrap(),
                    Err(_) => println!("Connexion Error"),
                },
                "-s" | "--search" => match search_package(package).await {
                    Ok(value) => println!("{}", value[0].0),
                    Err(_) => println!("Connexion Error"),
                },
                _ => println!("{HELP}"),
            }
        }
        Err(_) => println!("{HELP}"),
    };
}

#[tokio::main]
async fn main() {
    cli().await;
}
