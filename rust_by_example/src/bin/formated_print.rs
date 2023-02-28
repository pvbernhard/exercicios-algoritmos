// https://doc.rust-lang.org/stable/rust-by-example/hello/print.html

fn main() {
  let pi = 3.141592;
  // casas decimais no primeiro argumento
  println!("Pi é aproximadamente {pi:.0$}", 3);
  println!("Mais casas: {pi:.4}");
}