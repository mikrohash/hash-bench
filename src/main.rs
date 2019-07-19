pub mod data_structs;

use data_structs::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_t9_encoding_decoding() {
        let t9s : &[T9] = &T27::random().0;
        for &a in t9s.iter() {
            let b = a.encode().decode();
            assert_eq!([a.0, a.1, a.2], [b.0, b.1, b.2]);
        }
    }

    #[test]
    fn test_from_human_readable() {
        T721::test_from_human_readable();
        T243::test_from_human_readable();
        T81::test_from_human_readable();
        T27::test_from_human_readable();
    }

    #[test]
    fn test_tryte_array() {
        T27::test_tryte_array();
        T81::test_tryte_array();
        T243::test_tryte_array();
        T721::test_tryte_array();
    }

    #[test]
    fn test_encode_decode() {
        T27::test_encode_decode();
        T81::test_encode_decode();
        T243::test_encode_decode();
        T721::test_encode_decode();
    }
}