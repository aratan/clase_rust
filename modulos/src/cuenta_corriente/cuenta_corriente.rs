// ---------- Cuenta corriente (con descubierto) ----------
use crate::errores::{validar_cantidad, ErrorCuenta};
use crate::traits::Cuenta;

pub struct CuentaCorriente {
    saldo: f64,
    descubierto_max: f64,
}

impl CuentaCorriente {
    pub fn new(saldo: f64, descubierto_max: f64) -> Self {
        Self {
            saldo,
            descubierto_max,
        }
    }
}

impl Cuenta for CuentaCorriente {
    fn tipo(&self) -> &str {
        "Cuenta corriente"
    }

    fn saldo(&self) -> f64 {
        self.saldo
    }

    fn disponible(&self) -> f64 {
        self.saldo + self.descubierto_max
    }
    // () ? -> devuelve el error automáticamente si falla, así que no repites el if en cada método
    fn ingresar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        validar_cantidad(cantidad)?;
        self.saldo += cantidad;
        Ok(())
    }

    fn retirar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        validar_cantidad(cantidad)?;
        if cantidad > self.disponible() {
            return Err(ErrorCuenta::SaldoInsuficiente);
        }
        self.saldo -= cantidad; // el saldo puede quedar negativo
        Ok(())
    }
}
