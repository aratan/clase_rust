use cajero_poo_trait::cajero::Cajero;
use cajero_poo_trait::cuenta_ahorro::CuentaAhorro;
use cajero_poo_trait::cuenta_corriente::CuentaCorriente;
use cajero_poo_trait::traits::Cuenta;
use cajero_poo_trait::utilidades::leer_string;

fn elegir_cuenta() -> Box<dyn Cuenta> {
    loop {
        println!("Tipo de cuenta:");
        println!("1. Corriente (descubierto de 500€)");
        println!("2. Ahorro (2 % anual, 3 retiradas al mes)");
        match leer_string("Elige (1 o 2):").as_str() {
            "1" => return Box::new(CuentaCorriente::new(1000.0, 500.0)),
            "2" => return Box::new(CuentaAhorro::new(1000.0, 0.02, 3)),
            _ => println!("Opción no válida"),
        }
    }
}
// ¿dyn o genéricos? Depende de cuándo se conoce el tipo de cuenta:
//
// - Genéricos `Cajero<C: Cuenta>`: despacho estático. El compilador genera una
//   versión de Cajero por cada tipo que implemente Cuenta, las llamadas se
//   resuelven en compilación (se pueden inline) y no hay coste en ejecución.
//   A cambio, el tipo debe conocerse al compilar y cada Cajero<T> es un tipo
//   distinto (no se pueden mezclar cuentas en un Vec, por ejemplo).
// - `Box<dyn Cuenta>`: despacho dinámico. Un único tipo Cajero guarda en el heap
//   cualquier cuenta; cada llamada pasa por la vtable, con el coste de una
//   indirección y una reserva en el heap.
//
// Aquí el usuario elige la cuenta en ejecución con elegir_cuenta(), así que el
// tipo no se conoce al compilar y `dyn` es la opción adecuada. Los genéricos
// encajarían si el tipo se fijara al compilar, por ejemplo:
// `Cajero::new(CuentaAhorro::new(1000.0, 0.02, 3))`.
// struct Cajero<C: Cuenta> {
//     cuenta: C,
// }

fn main() {
    let mut cajero = Cajero::new(elegir_cuenta());
    cajero.ejecutar();
}
