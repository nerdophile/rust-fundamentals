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
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Media: {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audio: {}", title)
        // } else {
        //     String::from("Standard Media desc")
        // }
        match self {
            Media::Book { title, author } => { format!("Book: {} {}", title, author) }
            Media::Movie { title, director } => { format!("Media: {} {}", title, director) }
            Media::Audiobook { title } => { format!("Audio: {}", title) }
            Media::Podcast(episode_number) => { format!("Podcast: {}", episode_number) }
            Media::Placeholder => { format!("Placeholder") }
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
        self.items.push(media)
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index { Some(&self.items[index]) } else { None }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An audio book"),
    };

    let good_movie = Media::Movie {
        title: String::from("Inception"),
        director: String::from("Nolan"),
    };

    let bad_book = Media::Book {
        title: String::from("Half girlfriend"),
        author: String::from("Chetan Bhagat"),
    };

    println!("{}", audiobook.description());
    println!("{}", good_movie.description());
    println!("{}", bad_book.description());

    let mut catalog = Catalog::new();

    let joe_rogan = Media::Podcast(100);
    let placeholder = Media::Placeholder;

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(joe_rogan);
    catalog.add(placeholder);

    match catalog.get_by_index(1) {
        Some(value) => {
            println!("Item {:#?}", value);
        }
        None => {
            println!("No value here");
        }
    }

    let item = catalog.get_by_index(0);
    let placeholder = Media::Placeholder;

    println!("{:#?}", item.unwrap_or(&placeholder));
    main2()
}

// TODO:
// 1) Safely access the first account in the 'accounts' vector using the
//    .first_mut() method.
// 2) '.first_mut()' returns an Option whose Some variant is a mutable ref to
//     an Account. Use a 'match' statement to figure out if
//     you have a Some or a None
// 3) In the Some case, set the balance of the account to 30, then print the account
// 4) In the None case, print the message "No account found"
// Hint: You might have to add in the 'mut' keyword somewhere...

#[derive(Debug)]
struct Account {
    balance: i32,
}

fn main2() {
    let mut accounts: Vec<Account> = vec![Account { balance: 0 }, Account { balance: 10 }];

    match accounts.first_mut() {
        Some(account) => {
            account.balance = 30;

            println!("{:#?}", account);
        }
        None => {
            println!("No account found");
        }
    }
}
