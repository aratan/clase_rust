// ---------- Errores ----------
use std::fmt;

#[derive(Debug)]
pub enum ErrorCuenta {
    CantidadNoValida,
    SaldoInsuficiente,
    LimiteRetiradas(u32),
}

impl fmt::Display for ErrorCuenta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorCuenta::CantidadNoValida => write!(f, "La cantidad debe ser mayor que 0"),
            ErrorCuenta::SaldoInsuficiente => write!(f, "Saldo insuficiente"),
            ErrorCuenta::LimiteRetiradas(n) => {
                write!(f, "Has alcanzado el límite de {n} retiradas este mes")
            }
        }
    }
}

pub fn validar_cantidad(cantidad: f64) -> Result<(), ErrorCuenta> {
    if cantidad > 0.0 {
        Ok(())
    } else {
        Err(ErrorCuenta::CantidadNoValida)
    }
}
