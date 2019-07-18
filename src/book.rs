use std::collections::HashMap;


#[derive(Copy,Clone)]
pub struct Book<'a>{
    pub title: &'a str,
    pub pages: u32
}




#[derive(Clone)]
pub struct Shelf<'a>{
    pub id: u32,
    pub books: Option<HashMap<&'a str, Book<'a> >>
}

impl<'a> Shelf<'a>{

    pub fn add_book(&mut self, b: Book){
        let mut book_map = self.books.unwrap_or(HashMap::new());
        book_map.insert(b.title, b);
    }

/*
    pub fn add_book(&mut self, book: Book){
        if let Some(mut book_map) = self.books {
            book_map.insert(book.title, book);            
        }
        self.books = Some(HashMap::new());
        self.add_book(book);
    }
*/

}
