fn main() {
    // http get request
    let response = reqwest::blocking::get(
        "https://www.imdb.com/search/title/?groups=top_100&sort=user_rating,desc&count=100",
    )
    .unwrap()
    .text()
    .unwrap();

    // parse a string of html as a document
    let document = scraper::Html::parse_document(&response);

    // create a selector from a css query
    let title_selector = scraper::Selector::parse("h3.lister-item-header>a").unwrap();

    //  use the selector to return an iterator of elements, and map them by their inner html
    let titles = document.select(&title_selector).map(|x| x.inner_html());

    // print the rank and title of each movie in the top 100
    titles.zip(1..101).for_each(|(item, number)| println!("{}. {}", number, item));
}
