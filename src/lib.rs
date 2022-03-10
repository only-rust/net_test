use std::net::TcpStream;


/// Функция для проверки соединения с google.com
pub fn conn() {
  if let Ok(_) = TcpStream::connect("173.194.222.139:80") {
      println!("Connected to the server!");
  } else {
      println!("Couldn't connect to server...");
  }
}