mod content;

use content::media::Media;
use content::catalog::Catalog;


fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Se frem"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Some nice director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad title"),
        author: String::from("Some nice author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);


    let item = catalog.get_by_index(40);
    if let Some(value) = catalog.get_by_index(9999) {
        println!("Item in pattern match {:#?}", value)
    } else {
        println!("No value found whatsoever")
    }


    // println!("{:#?}", my_catalog);

    match catalog.get_by_index(0) {
        Some(value) => {
            print!("{:#?}", value)
        }
        None => {
            println!("NoValue available here.")
        }
    }
}
