#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}
impl Media {
   pub fn description(&self) -> String {
        /*
        if let Media::Book {title,author} = self{
        format!("Book:  {} {}", title, author)

        } else if let Media::Movie {title,director} = self{1
        format!("Movie {} {}", title, director)

        }
        else if let Media::Audiobook {title} = Self {
        format!("{}", title)
        } else {
        String::from("Lazy description")
        }
        */
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
