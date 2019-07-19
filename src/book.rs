use std::collections::HashMap;


#[derive(Clone, Debug)]
pub struct Book{
    pub title: String,
    pub pages: u32
}




#[derive(Clone, Debug)]
pub struct Shelf{
    #[deprecated(
        note = "id is not needed here, because shelfs id'ed with the key on the map"
    )]
    pub id: u32,
    pub books: Option<HashMap<String, Book>>
}

impl Shelf{

    pub fn add_book(&mut self, b: Book){

        let mut book_map = self.books.clone().unwrap();
        book_map.insert(b.title.clone(), b);
        self.books = Some(book_map);
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
