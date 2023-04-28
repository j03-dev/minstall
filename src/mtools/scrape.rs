use scraper::{Html, Selector};

pub fn get_element_inner_html<T: FromIterator<String>>(html_page: &Html, html_tag: &str) -> T {
    let selector: Selector = Selector::parse(html_tag).unwrap();
    html_page
        .select(&selector)
        .map(|x| x.inner_html())
        .collect()
}

pub fn get_element_attribute<T: FromIterator<String>>(
    html_page: &Html,
    html_tag: &str,
    attribute: &str,
) -> T {
    let selector: Selector = Selector::parse(html_tag).unwrap();
    html_page
        .select(&selector)
        .map(|x| x.value().attr(attribute).unwrap().to_string())
        .collect()
}
