use csv::{ReaderBuilder, StringRecord};
use std::fs;

const FILENAME: &str = "history.csv";



#[derive(Debug)]
struct DatoHistoria {
    tipo_dato: String,
    tag: String,
    texto: String,
    vida: i32,
}

fn main() {
    let mut datos_historia: Vec<DatoHistoria> = vec![];
    let content: String = fs::read_to_string(FILENAME).unwrap();
    let mut rdr = ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(content.as_bytes());

    for result in rdr.records() {
        // print!("Texto: {}", result.unwrap().get(2).unwrap().trim());
        let dato = DatoHistoria {
            tipo_dato: "Situacion".to_string(),
            tag: "INICIO".to_string(),
            texto: result.unwrap().get(2).unwrap().trim().to_string(),
            vida: 0,
        };
        datos_historia.push(dato);
    }

    

    println!("{:?}", datos_historia);
}
