// ---------- Cuenta de ahorro (intereses y límite de retiradas) ----------
use crate::errores::{validar_cantidad, ErrorCuenta};
use crate::traits::Cuenta;

pub struct CuentaAhorro {
    saldo: f64,
    interes_anual: f64, // 0.02 = 2 %
    retiradas_mes: u32,
    max_retiradas: u32,
}

impl CuentaAhorro {
    pub fn new(saldo: f64, interes_anual: f64, max_retiradas: u32) -> Self {
        Self {
            saldo,
            interes_anual,
            retiradas_mes: 0,
            max_retiradas,
        }
    }
}

impl Cuenta for CuentaAhorro {
    fn tipo(&self) -> &str {
        "Cuenta de ahorro"
    }

    fn saldo(&self) -> f64 {
        self.saldo
    }

    fn ingresar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        validar_cantidad(cantidad)?;
        self.saldo += cantidad;
        Ok(())
    }

    fn retirar(&mut self, cantidad: f64) -> Result<(), ErrorCuenta> {
        validar_cantidad(cantidad)?;
        if self.retiradas_mes >= self.max_retiradas {
            return Err(ErrorCuenta::LimiteRetiradas(self.max_retiradas));
        }
        if cantidad > self.saldo {
            return Err(ErrorCuenta::SaldoInsuficiente);
        }
        self.saldo -= cantidad;
        self.retiradas_mes += 1;
        Ok(())
    }

    fn fin_de_mes(&mut self) {
        let interes = self.saldo * self.interes_anual / 12.0;
        self.saldo += interes;
        self.retiradas_mes = 0;
        println!("Intereses abonados: {:.2}€", interes);
    }
}
