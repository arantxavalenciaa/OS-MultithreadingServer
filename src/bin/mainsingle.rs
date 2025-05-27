
use std::{
    fs,
    io::{prelude::*, BufReader, Write}, //Leer lo que llega de la conexión y escribir respuestas
    net::{TcpListener, TcpStream}, //Enviar y recibir por la red
};

//Crear el servidor
fn main() {
    let listener = TcpListener::bind("127.0.0.1:16834").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

//Respuesta
fn handle_connection(mut stream: TcpStream) {
    //Leer línea por línea
    let buf_reader = BufReader::new(&stream);

    //Leer lo que pide el navegador
    let request_lines: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // Mostrar en consola
    println!("Solicitud HTTP completa:");
    for line in &request_lines {
        println!("{}", line);
    }

    // Decidimos que archivo dar
    let request_line = &request_lines[0];

    //Primera linea de la solicitud, vemos que quiere la solicitud
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "Respuesta.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}


