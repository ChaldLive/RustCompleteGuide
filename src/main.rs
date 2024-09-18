#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}
impl Media {
    fn description(&self) -> String {
        // if let Media::Book {title,author} = self{
        //     format!("Book:  {} {}", title, author)
        //
        // } else if let Media::Movie {title,director} = self{1
        //     format!("Movie {} {}", title, director)
        //
        // }
        // else if let Media::Audiobook {title} = Self {
        //     format!("{}", title)
        // } else {
        //     String::from("Lazy description")
        // }
        match self {
            Media::Book { title, author } => {
                format!("Book:  {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie:  {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(episode_number) => {
                format!("Podcast episode: {}", episode_number)
            }
            Media::Placeholder => {
                "Placeholder".to_string()
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }
    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
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
