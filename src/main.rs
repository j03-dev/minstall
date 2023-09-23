// use std::process::exit;

use scraper::Html;

use crate::mtools::{downloader::download_package, exec::run_command, parse::parser, scrape};

mod mtools;

static LINUX_FROM_SCRATCH: &str = "https://www.linuxfromscratch.org/blfs/view/svn";
static HELP: &str = "
usage:
for install: minstall -i or --install <package_name>
for search package: minstall -s  or --search <package_name>
";


fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    let mut matrix: Vec<Vec<usize>> = vec![vec![0; len2 + 1]; len1 + 1];

    (0..=len1).for_each(|i| {
        matrix[i][0] = i;
    });

    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, char1) in s1.chars().enumerate() {
        for (j, char2) in s2.chars().enumerate() {
            let cost = if char1 == char2 { 0 } else { 1 };

            matrix[i + 1][j + 1] = std::cmp::min(
                matrix[i][j + 1] + 1,
                std::cmp::min(matrix[i + 1][j] + 1, matrix[i][j] + cost),
            );
        }
    }

    matrix[len1][len2]
}

#[derive(Debug)]
pub enum MinstallError {
    ConnexionError,
    InstallationError,
    BuildError,
}

type Error = MinstallError;

async fn search_package(package: String, distance: usize) -> Result<Vec<(String, String)>, Error> {
    let request = format!("{LINUX_FROM_SCRATCH}/index.html");
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
                    let package_name = name.split('-').collect::<Vec<&str>>()[0].to_string();
                    levenshtein_distance(&package_name.to_lowercase(), &package.to_lowercase()) <= distance
                })
                .collect())
        }
        Err(_) => Err(Error::ConnexionError),
    }
}

async fn install_the_package(link: &str) -> Result<(), Error> {
    let link = format!("{LINUX_FROM_SCRATCH}/{link}");
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
                    .map(|c| c.to_string().replace(['\n','\\'], ""))
                    .collect::<Vec<String>>();
                let install = &build_command[1];

                println!("try to download: {url}");
                download_package(url).await.expect("download failed");

                let file_name = {
                    let urls = url.split('/').collect::<Vec<_>>();
                    urls.last().unwrap().to_string()
                };

                run_command("cd packages")
                    .expect("failed to change directory to inside the packages directory");

                let tar_cmd = format!("tar -xvf {file_name}");
                run_command(&tar_cmd).expect("failed to extrat package file");

                let dir = format!("cd {dir}", dir = file_name.replace(".tar.xz", ""));
                run_command(&dir).expect(
                    "failed to change the current directory to inside the extract directory",
                );

                // etap build
                for cmd in build {
                    if run_command(cmd).is_err() {
                        return Err(Error::BuildError);
                    }
                }
                // install the package
                if run_command(install).is_err() {
                    return Err(Error::InstallationError);
                }
            }
            Ok(())
        }
        Err(_) => Err(Error::ConnexionError),
    }
}

async fn cli() {
    match parser() {
        Ok(output) => {
            let (args, package) = output;
            match args.as_str() {
                "-i" | "--install" => match search_package(package.clone(), 0).await {
                    Ok(value) => {
                        if let Some(v) = value.get(0) {
                            install_the_package(&v.1).await.unwrap()
                        } else {
                            println!("Package '{package}' was not found")
                        }
                    },
                    Err(err) => eprintln!("{err:?}"),
                },
                "-s" | "--search" => match search_package(package.clone(), 2).await {
                    Ok(value) => {
                        for result in value {
                            println!("{r} => {LINUX_FROM_SCRATCH}/{l}", r = result.0, l = result.1);
                        }
                    }
                    Err(err) => eprintln!("{err:?}"),
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
