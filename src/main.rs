// use std::option::Option;
// use std::option::Option::Some;
use std::string::ToString;
use std::vec::Vec;

#[derive(Debug)]
enum Media{
    Book {title: String, author: String},
    Movie{title: String, director: String},
    Audiobook{title: String},
    Podcast(u32),
    Placeholder
}
impl Media {
    fn description(&self) -> String{
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
    items: Vec<Media>
}

impl Catalog{
    fn new() -> Self{
        Catalog{ items: vec![]}
    }
    fn add(&mut self, media: Media){
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> MyIndexValidation{
        if index < self.items.len() {
            MyIndexValidation::ValueAtIdex(&self.items[index])
        } else {
            MyIndexValidation::NoValue
        }
    }
}
// With lifetime anotation
enum MyIndexValidation<'a> {
    ValueAtIdex(&'a Media),
    NoValue
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

    let mut my_catalog = Catalog::new();
    my_catalog.add(audiobook);
    my_catalog.add(good_movie);
    my_catalog.add(bad_book);
    my_catalog.add(podcast);
    my_catalog.add(placeholder);

    /*      Important  Note on Enums
        Working with enums forces the programmer to think in specific terms as Rust does not
        have the idea of null, nil or undefined.
        Instead, addressing e.g. a value out of index in the catalog, yields a standard
        language built in value  of enum Option. Containing two values Some or none.
        Some means you have something, none indicating out of range problem, in this particular
        case.
     */
    // match my_catalog.items.get(0) {
    //     Option::Some(value) =>{
    //         println!("Item: {:#?}",value)
    //     }
    //     Option::None =>{
    //         println!("Nothing at that specific index")
    //     }
    // }


    // let item = my_catalog.get_by_index(40);
    // println!("{:#?}", my_catalog);

    match my_catalog.get_by_index(0){
        MyIndexValidation::ValueAtIdex(media) => {
            print!("{:#?}", media);
        }
        MyIndexValidation::NoValue => {
            println!("NoValue available here.");
        }
    }
}
