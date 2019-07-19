use std::collections::HashMap;


#[derive(Clone)]
pub struct Book{
    pub title: String,
    pub pages: u32
}


#[derive(Clone)]
pub struct Shelf{
    pub id: u32,
    pub books: Option<HashMap<String, Book >>
}

impl Shelf{

    pub fn add_book(&mut self, b: Book){
        let mut book_map = self.books.clone().unwrap();
        book_map.insert(b.title.clone(),b);
        self.books = Some(book_map);
    }

}
