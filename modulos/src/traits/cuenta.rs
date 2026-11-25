// ---------- Trait ----------
// Trait con métodos obligatorios y por defecto:
// disponible() y fin_de_mes() tienen implementación por defecto.
// CuentaCorriente sobrescribe disponible() y CuentaAhorro sobrescribe fin_de_mes()
use crate::errores::ErrorCuenta;

pub trait Cuenta {
    // Métodos obligatorios: cada cuenta los implementa a su manera
    fn tipo(&self) -> &str;
    fn saldo(&self) -> f64;
    fn ingresar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta>;
    fn retirar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta>;

    // Métodos con implementación por defecto: se pueden sobrescribir
    fn disponible(&self) -> f64 {
        self.saldo()
    }

    fn fin_de_mes(&mut self) {
        // por defecto no hace nada
    }
}
