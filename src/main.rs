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
  // Podcast {
  //   title: String,
  //   episode_number: u32,
  // },
  Podcast(u32),
  Placeholder
}

impl Media {
  fn description(&self) -> String {
    // if let Media::Book { title, author } = self {
    //   format!("Book: {} {}", title, author)
    // }
    // else if let Media::Movie { title, director } = self {
    //   format!("Movie: {} {}", title, director)
    // }
    // else if let Media::Audiobook { title } = self {
    //   format!("Audiobook: {}", title)
    // }
    // else {
    //   String::from("Media description")
    // }

    match self {
      Media::Book { title, author } => format!("Book: {} {}", title, author),
      Media::Movie { title, director } => format!("Movie: {} {}", title, director),
      Media::Audiobook { title } => format!("Audiobook: {}", title),
      Media::Podcast(id) => format!("Podcast: {}", id),
      Media::Placeholder => "Placeholder".to_string(),
    }
  }
}

#[derive(Debug)]
struct Catalog {
  items: Vec<Media>,
}

impl Catalog {
  fn new() -> Self {
    return Catalog { items: vec![] };
  }

  fn add(&mut self, media: Media) {
    self.items.push(media);
  }

  // fn getByIndex(&self, index: usize) -> MaybeMedia {
  //   if self.items.len() > index {
  //     return MaybeMedia::HasValue(&self.items[index]);
  //   }
  //   else {
  //     return MaybeMedia::NoValue;
  //   }
  // }
  fn getByIndex(&self, index: usize) -> Option<&Media> {
    if self.items.len() > index {
      return Some(&self.items[index]);
    }
    else {
      return None;
    }
  }
}

#[derive(Debug)]
enum MaybeMedia<'a> {
  HasValue(&'a Media),
  NoValue
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

  let podcast = Media::Podcast(123);

  let placeholder = Media::Placeholder;

  // println!("{}", audiobook.description());
  // println!("{}", good_movie.description());
  // println!("{}", bad_book.description());

  let mut catalog = Catalog::new();
  catalog.add(audiobook);
  catalog.add(good_movie);
  catalog.add(bad_book);
  catalog.add(podcast);
  catalog.add(placeholder);

  // match catalog.items.get(0) {
  //   Option::Some(value) => println!("{:#?}", value),
  //   Option::None => println!("nothing at index"),
  // }
  // match catalog.getByIndex(0) {
  //   MaybeMedia::HasValue(value) => println!("{:#?}", value),
  //   MaybeMedia::NoValue => println!("nothing at index"),
  // }
  match catalog.getByIndex(0) {
    Some(value) => println!("{:#?}", value),
    None => println!("nothing at index"),
  }

  // if let MaybeMedia::HasValue(value) = catalog.getByIndex(1110) {
  //   println!("Item in pattern match: {:#?}", value)
  // }
  // else {
  //   println!("No item available.")
  // }
  if let Some(value) = catalog.getByIndex(1110) {
    println!("Item in pattern match: {:#?}", value)
  }
  else {
    println!("No item available.")
  }

  // println!("{:#?}", catalog.getByIndex(0));
  // println!("{:#?}", catalog.getByIndex(40));
}
