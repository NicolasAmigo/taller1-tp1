use std::collections::HashMap;

#[derive(Debug)]
/// Representacion de un tablero de buscaminas despues de aplicar su transformacion.
/// Solo se puede crear procesando un TableroSinProcesar
pub struct TableroProcesado {
    pub tablero: HashMap<(i32, i32), u8>,
}

impl TableroProcesado {
    pub fn imprimir(self) {
        let mut x = 0;
        let mut y = 0;

        while self.tablero.get(&(x, y)) != Option::None {
            while self.tablero.get(&(x, y)) != Option::None {
                print!("{}", *self.tablero.get(&(x, y)).unwrap());
                y = y + 1;
            }
            println!("");
            x = x + 1;
            y = 0;
        }
    }
    #[allow(dead_code)]
    pub fn get(&self, x: i32, y: i32) -> Option<&u8> {
        self.tablero.get(&(x, y))
    }
}
