// ---------- Cajero ----------
use crate::traits::Cuenta;
use crate::utilidades::{leer_numero_f64_option, leer_string};

pub struct Cajero {
    cuenta: Box<dyn Cuenta>, // cualquier tipo que implemente Cuenta
}

impl Cajero {
    /*
    Box<dyn Cuenta>: es un trait object.
    El tamaño de cada struct es distinto, así que hay que ponerlos en el heap detrás de un puntero.
    El coste es una llamada indirecta (vtable).
    Se usa dyn porque la cuenta se elige en ejecución; la comparación con la
    alternativa genérica (despacho estático) está documentada en main.rs.
    */
    pub fn new(cuenta: Box<dyn Cuenta>) -> Self {
        Self { cuenta }
    }

    fn mostrar_menu(&self) {
        println!("\n--- Cajero: {} ---", self.cuenta.tipo());
        println!("1. Consultar saldo");
        println!("2. Ingresar");
        println!("3. Retirar efectivo");
        println!("4. Simular fin de mes");
        println!("5. Salir");
    }

    fn consultar_saldo(&self) {
        println!("Saldo actual: {:.2}€", self.cuenta.saldo());
        if self.cuenta.disponible() != self.cuenta.saldo() {
            println!("Disponible:   {:.2}€", self.cuenta.disponible());
        }
    }

    fn ingresar(&mut self) {
        match leer_numero_f64_option("¿Cuánto quieres ingresar?") {
            Some(c) => match self.cuenta.ingresar(c) {
                Ok(()) => self.consultar_saldo(),
                Err(e) => println!("{e}"),
            },
            None => println!("Introduce un número válido"),
        }
    }

    fn retirar(&mut self) {
        match leer_numero_f64_option("¿Cuánto quieres sacar?") {
            Some(c) => match self.cuenta.retirar(c) {
                Ok(()) => self.consultar_saldo(),
                Err(e) => println!("{e}"),
            },
            None => println!("Introduce un número válido"),
        }
    }

    pub fn ejecutar(&mut self) {
        loop {
            self.mostrar_menu();
            match leer_string("Selecciona una opción (1-5):").as_str() {
                "1" => self.consultar_saldo(),
                "2" => self.ingresar(),
                "3" => self.retirar(),
                "4" => {
                    self.cuenta.fin_de_mes();
                    self.consultar_saldo();
                }
                "5" => {
                    println!("Hasta la próxima");
                    break;
                }
                _ => println!("Opción no válida"),
            }
        }
    }
}
