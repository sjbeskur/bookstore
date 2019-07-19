mod book;

use book::*;
use std::collections::HashMap;

fn main() {
    
    let mut shelfs: HashMap<u32,&Shelf> = HashMap::new();

    let b1 = Book { title: "Learn C#".to_string()  , pages: 100 };
    let b2 = Book { title: "Learn Java".to_string(), pages: 150 };
    let b3 = Book { title: "Learn Rust".to_string(), pages: 100000 };

    let mut empty_shelf = book::Shelf{ id: 1, books: None };
    empty_shelf.add_book(b1);
    empty_shelf.add_book(b2);
    empty_shelf.add_book(b3);

    shelfs.insert(empty_shelf.id, &empty_shelf);

    let id = 1;
    if let Some(shelf) = shelfs.get(&id){
        let mut s = (*shelf).clone();

        if s.books.is_none(){
            s.books = Some(HashMap::new());
            shelfs.insert(id, &s);
        }
    }
}