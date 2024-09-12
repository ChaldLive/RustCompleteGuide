use std::slice::IterMut;

#[derive(Debug)]
enum Media{
    Book {title: String, author: String},
    Movie{title: String, director: String},
    Audiobook{title: String},
}
impl Media {
    fn description(&self) -> String{
        // if let Media::Book {title,author} = self{
        //     format!("Book:  {} {}", title, author)
        //
        // } else if let Media::Movie {title,director} = self{
        //     format!("Movie {} {}", title, director)
        //
        // }
        // else if let Media::Audiobook {title} = Self {
        //     format!("{}", title)
        // } else {
        //     String::from("Lazy description")
        // }
        match self{
            Media::Book {title,author} => {
                format!("Book:  {} {}", title, author)
            }
            Media::Movie {title,director} => {
                format!("Movie:  {} {}", title, director)
            }
            Media::Audiobook {title} => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

struct Catalog {
    items: Vec<Media>
}

impl Catalog{
    fn new() -> Self{
        Catalog{ items: vec![]}
    }
    fn add(&mut self, media: Media){
        self.items.push(media);
    }
}

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

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());
}
