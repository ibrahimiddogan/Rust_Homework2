use std::io;
fn main() {
    println!("Kelime sayısını öğrenmek istediğiniz cümleyi yazınız :");
    let mut alici = String::new();
    io::stdin().read_line(&mut alici).expect("Giriş okuma hatası");
    let word_counter = WordCounter{
        text:alici
    };
    let word_count = word_counter.count_words();
    println!("Kelime sayısı {}",word_count);
}
struct WordCounter{
    text:String
}

impl WordCounter {
    fn new(text:&str)->WordCounter {
        WordCounter {
            text: String::from(text),
        }
    }
}
impl WordCounter {
    fn count_words(&self)->usize {

        if self.text.trim().is_empty() {
            println!("Hata: Boş bir metin girdiniz!");
            return 0;
        }


        else {self.text.split_whitespace().count()}
    }
}
