// Utilidades de entrada por teclado.
use std::io::{self, Write};

/// Muestra el mensaje y devuelve la línea escrita, sin espacios sobrantes.
pub fn leer_string(mensaje: &str) -> String {
    print!("{mensaje} ");
    io::stdout().flush().expect("no se pudo mostrar el mensaje");
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("no se pudo leer la entrada");
    entrada.trim().to_string()
}

/// Pide un número decimal. Devuelve None si lo escrito no es un número válido.
pub fn leer_numero_f64_option(mensaje: &str) -> Option<f64> {
    leer_string(mensaje).parse().ok()
}
