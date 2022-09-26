#[cfg(test)]
mod tests {
    use std::fs::{read_to_string, remove_file, File};

    #[test]
    fn test_archivo_no_existe_falla() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"noexiste.txt".to_string());
        assert!(tablero.is_err());
    }

    #[test]
    fn test_archivo_existe() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/testunit1.txt".to_string());
        assert!(!tablero.is_err());
    }

    #[test]
    fn test_tablero_sin_procesar_correcto() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/testunit1.txt".to_string());

        assert!(!tablero.is_err());

        let tablero = tablero.unwrap();

        // .
        assert_eq!(*tablero.get(0, 0).unwrap(), 46u8);
        // *
        assert_eq!(*tablero.get(1, 0).unwrap(), 42u8);
        // *
        assert_eq!(*tablero.get(2, 0).unwrap(), 42u8);
    }

    #[test]
    fn test_tablero_procesado_es_correcto() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/testunit1.txt".to_string());

        assert!(!tablero.is_err());

        let tablero = tablero.unwrap().procesar();

        // 1
        assert_eq!(*tablero.get(0, 0).unwrap(), 49u8);
        // *
        assert_eq!(*tablero.get(1, 0).unwrap(), 42u8);
        // *
        assert_eq!(*tablero.get(2, 0).unwrap(), 42u8);
    }

    #[test]
    fn test_tablero_procesado_es_correcto2() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/testunit2.txt".to_string());

        assert!(!tablero.is_err());

        let tablero = tablero.unwrap().procesar();

        // si falla en unwrap quiero que el test falle
        assert_eq!(*tablero.get(0, 0).unwrap(), 50u8); // 2
        assert_eq!(*tablero.get(1, 0).unwrap(), 42u8); // *
        assert_eq!(*tablero.get(2, 0).unwrap(), 51u8); // 3
        assert_eq!(*tablero.get(3, 0).unwrap(), 42u8); // *
        assert_eq!(*tablero.get(0, 1).unwrap(), 42u8); // *
        assert_eq!(*tablero.get(1, 1).unwrap(), 52u8); // 4
        assert_eq!(*tablero.get(2, 1).unwrap(), 42u8); // *
        assert_eq!(*tablero.get(3, 1).unwrap(), 42u8); // *
        assert_eq!(*tablero.get(0, 2).unwrap(), 50u8); // 2
        assert_eq!(*tablero.get(1, 2).unwrap(), 42u8); // *
        assert_eq!(*tablero.get(2, 2).unwrap(), 50u8); // 2
        assert_eq!(*tablero.get(3, 2).unwrap(), 49u8); // 1
    }

    #[test]
    fn test_integracion1() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/test1.txt".to_string());
        let tablero = match tablero {
            Err(_) => panic!("Error al procesar el tablero"),
            Ok(x) => x.procesar(),
        };
        match File::create("out1.txt") {
            Ok(x) => {
                if let Err(_) = tablero.imprimir(x) {
                    panic!("Error al escribir archivo");
                }
            }
            Err(_) => panic!("Error al crear archivo"),
        };
        let out = match read_to_string("out1.txt") {
            Err(_) => panic!("Error al abrir out text"),
            Ok(x) => x,
        };

        let esperado = "...\n...\n...\n";

        assert_eq!(out, esperado);
        //Elimino el archivo creado
        if let Err(_) = remove_file("out1.txt") {
            panic!("Error al borrar el archivo");
        };
    }
    #[test]
    fn test_integracion2() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/test2.txt".to_string());
        let tablero = match tablero {
            Err(_) => panic!("Error al procesar el tablero"),
            Ok(x) => x.procesar(),
        };
        match File::create("out2.txt") {
            Ok(x) => {
                if let Err(_) = tablero.imprimir(x) {
                    panic!("Error al escribir archivo");
                }
            }
            Err(_) => panic!("Error al crear archivo"),
        };
        let out = match read_to_string("out2.txt") {
            Err(_) => panic!("Error al abrir out text"),
            Ok(x) => x,
        };
        let esperado = ".1.\n1*1\n.1.\n";

        assert_eq!(out, esperado);
        //Elimino el archivo creado
        if let Err(_) = remove_file("out2.txt") {
            panic!("Error al borrar el archivo");
        };
    }
    #[test]
    fn test_integracion3() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/test3.txt".to_string());
        let tablero = match tablero {
            Err(_) => panic!("Error al procesar el tablero"),
            Ok(x) => x.procesar(),
        };
        match File::create("out3.txt") {
            Ok(x) => {
                if let Err(_) = tablero.imprimir(x) {
                    panic!("Error al escribir archivo");
                }
            }
            Err(_) => panic!("Error al crear archivo"),
        };
        let out = match read_to_string("out3.txt") {
            Err(_) => panic!("Error al abrir out text"),
            Ok(x) => x,
        };
        let esperado = "2*2\n***\n2*2\n";

        assert_eq!(out, esperado);
        //Elimino el archivo creado
        if let Err(_) = remove_file("out3.txt") {
            panic!("Error al borrar el archivo");
        };
    }
}
