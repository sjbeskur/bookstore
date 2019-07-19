
mod book;

use book::*;
use std::collections::HashMap;

fn main() {
    
    //think: SNMP packet storage.
    let mut shelfs: HashMap<u32,Shelf> = HashMap::new();


    {
        //think: here comes new packet
        let b1 = Book { title: "Learn C#".into(),   pages: 100 };
        //think:add this packet to the storage
        add_book(1, &b1, &mut shelfs);  
    }





    let b2 = Book { title: "Learn Java".into(), pages: 150 };
    add_book(1, &b2, &mut shelfs);

    let b3 = Book { title: "Learn Rust".into(), pages: 100000 };
    add_book(2, &b3, &mut shelfs);

    println!("{:#?}", shelfs);
}



fn add_book(shelf_id:u32, book:&Book, shelfs:&mut HashMap<u32, Shelf>){
    if let Some(shelf) = shelfs.get_mut(&shelf_id){
        let mut books:HashMap<String,Book> = 
            if let Some(books) = shelf.books.clone(){ //this will disconnect map (books) from shelf
                books
            }else{
                HashMap::new()
            };
        books.insert(book.title.clone(), book.clone()); //adds book to books
        shelf.books = Some(books); // sets new val(some books) to shelf

    }else{
        let mut books = HashMap::new();
        books.insert(book.title.clone(), book.clone());
        let shelf = book::Shelf{ id: shelf_id, books: Some(books) };
        shelfs.insert(shelf_id, shelf);

    }




}