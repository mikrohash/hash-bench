#![allow(dead_code)]

#[derive(Copy, Clone)]
struct T9(u8, u8, u8);

#[derive(Copy, Clone)]
struct B16(u8, u8);

impl T9 {
    fn encode(&self) -> B16 {
        B16(self.0 + 27*(self.2%3), self.1 + 27*(self.2/3))
    }

    fn from_human_readable(c0 : char, c1 : char, c2 : char) -> Self {
        fn normalize_human_char(c : char) -> u8 {
            match c { '9' => 0,  _ => (c as u8)-('A' as u8)+1 }
        }
        T9(normalize_human_char(c0), normalize_human_char(c1), normalize_human_char(c2))
    }
}

impl B16 {
    fn decode(&self) -> T9 {
        T9(self.0%27, self.1%27, self.0/27 + 3*(self.1/27))
    }
}

struct T243([T9; 27]);
struct B432([B16; 27]);

impl T243 {
    fn encode(&self) -> B432 {
        let mut t_iter = self.0.iter();
        let mut encoded = B432([B16(0, 0); 27]);
        let mut b_iter = encoded.0.iter_mut();
        for _ in 0..27 {
            let t9 = t_iter.next().unwrap();
            let b16 = b_iter.next().unwrap();
            *b16 = t9.encode();
        }
        encoded
    }

    fn from_human_readable(s : &str) -> Self {
        let mut t243 = T243([T9(0, 0, 0); 27]);
        let mut src_iter = s.chars();
        let mut t243_iter = t243.0.iter_mut();
        for _ in 0..(s.len()+2)/3 {
            let c0 = src_iter.next().unwrap_or('9');
            let c1 = src_iter.next().unwrap_or('9');
            let c2 = src_iter.next().unwrap_or('9');
            *t243_iter.next().unwrap() = T9::from_human_readable(c0, c1, c2);
        }
        t243
    }

    fn as_tryte_array(&self) -> [u8; 81] {
        let mut array = [0 as u8; 81];
        let mut array_iter_mut = array.iter_mut();
        let mut t9_iter = self.0.iter();
        for _ in 0 .. 27 {
            let t9 = t9_iter.next().unwrap();
            *array_iter_mut.next().unwrap() = t9.0;
            *array_iter_mut.next().unwrap() = t9.1;
            *array_iter_mut.next().unwrap() = t9.2;
        }
        array
    }
}

impl B432 {
    fn decode(&self) -> T243 {
        let mut b_iter = self.0.iter();
        let mut decoded = T243([T9(0, 0, 0); 27]);
        let mut t_iter_mut = decoded.0.iter_mut();
        for _ in 0..27 {
            let b16 = b_iter.next().unwrap();
            let t9 = t_iter_mut.next().unwrap();
            *t9 = b16.decode();
        }
        decoded
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_t9_encoding_decoding() {
        // todo randomize
        let a = T9(4, 7, 13);
        let b = a.encode().decode();
        assert_eq!([a.0, a.1, a.2], [b.0, b.1, b.2]);
    }

    #[test]
    fn test_243_encoding_decoding() {
        let t243 = T243::from_human_readable("AB9C");
        let b432 = t243.encode();
        let t243 = b432.decode();

        let mut expected = [0 as u8; 81];
        expected[0] = 1;
        expected[1] = 2;
        expected[3] = 3;
        assert_eq!(expected[0..81], t243.as_tryte_array()[0..81]);
    }
}