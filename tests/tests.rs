#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/test1.txt".to_string());
        let tablero = tablero.procesar();

        assert_eq!(*tablero.get(0, 0).unwrap(), 46u8);
        assert_eq!(*tablero.get(0, 1).unwrap(), 46u8);
        assert_eq!(*tablero.get(0, 2).unwrap(), 46u8);
        assert_eq!(*tablero.get(1, 0).unwrap(), 46u8);
        assert_eq!(*tablero.get(1, 1).unwrap(), 46u8);
        assert_eq!(*tablero.get(1, 2).unwrap(), 46u8);
        assert_eq!(*tablero.get(2, 0).unwrap(), 46u8);
        assert_eq!(*tablero.get(2, 1).unwrap(), 46u8);
        assert_eq!(*tablero.get(2, 2).unwrap(), 46u8);
    }
    #[test]
    fn test2() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/test2.txt".to_string());
        let tablero = tablero.procesar();

        assert_eq!(*tablero.get(0, 0).unwrap(), 46u8);
        assert_eq!(*tablero.get(0, 1).unwrap(), 49u8);
        assert_eq!(*tablero.get(0, 2).unwrap(), 46u8);
        assert_eq!(*tablero.get(1, 0).unwrap(), 49u8);
        assert_eq!(*tablero.get(1, 1).unwrap(), 42u8);
        assert_eq!(*tablero.get(1, 2).unwrap(), 49u8);
        assert_eq!(*tablero.get(2, 0).unwrap(), 46u8);
        assert_eq!(*tablero.get(2, 1).unwrap(), 49u8);
        assert_eq!(*tablero.get(2, 2).unwrap(), 46u8);
    }
    #[test]
    fn test3() {
        let tablero =
            tp1::tablero_sin_procesar::TableroSinProcesar::new(&"tests/test3.txt".to_string());
        let tablero = tablero.procesar();

        assert_eq!(*tablero.get(0, 0).unwrap(), 50u8);
        assert_eq!(*tablero.get(0, 1).unwrap(), 42u8);
        assert_eq!(*tablero.get(0, 2).unwrap(), 50u8);
        assert_eq!(*tablero.get(1, 0).unwrap(), 42u8);
        assert_eq!(*tablero.get(1, 1).unwrap(), 42u8);
        assert_eq!(*tablero.get(1, 2).unwrap(), 42u8);
        assert_eq!(*tablero.get(2, 0).unwrap(), 50u8);
        assert_eq!(*tablero.get(2, 1).unwrap(), 42u8);
        assert_eq!(*tablero.get(2, 2).unwrap(), 50u8);
    }
}
