
mod book;

use book::*;
use std::collections::HashMap;

fn main() {
    
    let mut shelfs: HashMap<u32,&Shelf> = HashMap::new();

    let b1 = Book { title: "Learn C#",   pages: 100 };
    let b2 = Book { title: "Learn Java", pages: 150 };
    let b3 = Book { title: "Learn Rust", pages: 100000 };

    let mut empty_shelf = book::Shelf{ id: 1, books: None };

    shelfs.insert(empty_shelf.id, &empty_shelf);

    // This is failed attempt
    // cannot add books to my shelf ?
    /*
    empty_shelf.add_book(b1);
    empty_shelf.add_book(b2);
    empty_shelf.add_book(b3);
    */

    let id = 1;
    let mut shelf1 = shelfs.get_mut(&id).unwrap(); // id of the empty shelf

    // try without OO style impl
    if shelf1.books.is_none(){
        shelf1.books = Some(HashMap::new());
        
    }



//    if let Some(mut book_map) = shelf.books{
//        book_map.insert(b1.title.clone(), b1.clone());
//    }
    

}
