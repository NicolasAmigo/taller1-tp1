use std::collections::HashMap;
use std::fs::read_to_string;

use crate::tablero_procesado;
//'.'
const PUNTO: u8 = 46;
//'*'
const ASTER: u8 = 42;
// \n
const CR: u8 = 13;
const CERO: u8 = 48;

#[derive(Debug)]
/// Representacion de un tablero de buscaminas antes de aplicarse su transformacion
pub struct TableroSinProcesar {
    tablero: HashMap<(i32, i32), u8>,
}

impl TableroSinProcesar {
    pub fn new(file_path: &String) -> TableroSinProcesar {
        let contents = read_to_string(file_path).expect("Error al abrir el archivo");
        let mut tablero = HashMap::new();
        let mut x = 0;
        let mut y = 0;

        for val in contents.as_bytes() {
            match *val {
                CR => {
                    x = 0;
                    y += 1;
                }
                ASTER | PUNTO => {
                    tablero.insert((x as i32, y as i32), *val);
                    x += 1;
                }
                // Ingoro LF
                _ => {}
            }
        }
        TableroSinProcesar { tablero }
    }

    pub fn procesar(self) -> tablero_procesado::TableroProcesado {
        let mut tablero = HashMap::new();

        for ((x, y), val) in self.tablero.iter() {
            match *val {
                ASTER => {
                    tablero.insert((*x, *y), *val);
                }
                _ => {
                    //Siemre deberia ser un numero
                    let cant = self.cant_adya(*x, *y);

                    match cant {
                        0 => {
                            tablero.insert((*x, *y), PUNTO);
                        }
                        _ => {
                            // Cant + cero en ascii resulta en cant en ascii
                            tablero.insert((*x, *y), cant + CERO);
                        }
                    }
                }
            }
        }
        tablero_procesado::TableroProcesado { tablero }
    }

    fn cant_adya(&self, x: i32, y: i32) -> u8 {
        //Tomo '.' en el caso que no exista esa posicion
        let values = vec![
            self.tablero.get(&(x + 1, y)).unwrap_or(&PUNTO),
            self.tablero.get(&(x - 1, y)).unwrap_or(&PUNTO),
            self.tablero.get(&(x, y + 1)).unwrap_or(&PUNTO),
            self.tablero.get(&(x, y - 1)).unwrap_or(&PUNTO),
        ];
        values.iter().filter(|&c| **c == ASTER).count() as u8
    }
}
