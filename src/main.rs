#[derive(Debug)]
enum Media {
  Book {
    title: String,
    author: String,
  },
  Movie {
    title: String,
    director: String,
  },
  Audiobook {
    title: String,
  },
}

impl Media {
  fn description(&self) -> String {
    // return String::from(self.description())
    // return String::from("Media Description")
    if let Media::Book { title, author } = self {
      format!("Book: {} {}", title, author)
    }
    else if let Media::Movie { title, director } = self {
      format!("Movie: {} {}", title, director)
    }
    else if let Media::Audiobook { title } = self {
      format!("Audiobook: {}", title)
    }
    else {
      String::from("Media description")
    }
  }
}

fn main() {
  let audiobook = Media::Audiobook {
      title: String::from("An Audiobook"),
  };
  let good_movie = Media::Movie {
      title: String::from("Good Movie"),
      director: String::from("Good Director"),
  };
  let bad_book = Media::Book {
      title: String::from("Bad Book"),
      author: String::from("Bad Author"),
  };

  println!("{}", audiobook.description());
  println!("{}", good_movie.description());
  println!("{}", bad_book.description());
}
