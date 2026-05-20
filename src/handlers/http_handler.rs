use crate::types::Handler;
use std::net::TcpStream;
use std::io::Write;

pub enum HttpMethod { GET, POST }

pub struct HttpHandler {
    method: HttpMethod,
    host: &'static str,
    port: &'static str,
    path: &'static str,
}

impl HttpHandler {
    pub fn new(method: HttpMethod, host: &'static str, port: &'static str, path: &'static str) -> Self {
        Self { method, host, port, path }
    }
}

impl Handler for HttpHandler {
    fn execute(&self, msg: &str, trigger_value: i64) {
        // Конструируем адрес "host:port"
        let addr = format!("{}:{}", self.host, self.port);
        
        // Открываем чистый TCP-стрим к веб-серверу
        if let Ok(mut stream) = TcpStream::connect(addr) {
            // Форматируем тело запроса или параметры (значение переводим во float для UI)
            let body = format!("{{\"msg\":\"{}\",\"val\":{:.2}}}", msg, trigger_value as f64 / 100.0);
            let body_bytes = body.as_bytes();

            // Выбираем метод запроса
            let method_str = match self.method {
                HttpMethod::GET => "GET",
                HttpMethod::POST => "POST",
            };

            // Собираем честный HTTP/1.1 пакет вручную прямо в буфер сокета (0% кучи на парсеры)
            let mut request = Vec::with_capacity(512);
            let _ = write!(request, "{} {} HTTP/1.1\r\n", method_str, self.path);
            let _ = write!(request, "Host: {}\r\n", self.host);
            let _ = write!(request, "Content-Type: application/json\r\n");
            let _ = write!(request, "Content-Length: {}\r\n", body_bytes.len());
            let _ = write!(request, "Connection: close\r\n\r\n");
            
            // Если это POST, дописываем тело запроса
            if let HttpMethod::POST = self.method {
                request.extend_from_slice(body_bytes);
            }

            // Выплевываем HTTP-запрос в сеть
            let _ = stream.write_all(&request);
            let _ = stream.flush();
            println!("  [HTTP ACTION] -> Запрос {} отправлен на {}{}", method_str, self.host, self.path);
        } else {
            println!("  [HTTP ERROR] -> Не удалось подключиться к веб-серверу {}:{}", self.host, self.port);
        }
    }
}
