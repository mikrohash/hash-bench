#![allow(dead_code)]

#[derive(Copy, Clone)]
pub struct T9(pub u8, pub u8, pub u8);

#[derive(Copy, Clone)]
pub struct B16(pub u8, pub u8);

impl T9 {
    pub fn encode(&self) -> B16 {
        B16(self.0 + 27*(self.2%3), self.1 + 27*(self.2/3))
    }

    pub fn from_human_readable(c0 : char, c1 : char, c2 : char) -> Self {
        fn normalize_human_char(c : char) -> u8 {
            match c { '9' => 0,  _ => (c as u8)-('A' as u8)+1 }
        }
        T9(normalize_human_char(c0), normalize_human_char(c1), normalize_human_char(c2))
    }
}

impl B16 {
    pub fn decode(&self) -> T9 {
        T9(self.0%27, self.1%27, self.0/27 + 3*(self.1/27))
    }
}

macro_rules! trinary_struct {
    ($tname : ident, $trit_size : expr, $bname : ident) => {
        pub struct $tname(pub [T9; $trit_size/9]);

        impl $tname {

            pub fn random() -> Self {
                $tname::from_tryte_array($tname::random_tryte_array())
            }

            fn random_tryte_array() -> [u8; $trit_size/3] {
                use rand::Rng;
                let mut ring = rand::thread_rng();
                let mut random_tryte_array = [0 as u8; $trit_size/3];
                for i in 0..random_tryte_array.len() {
                    random_tryte_array[i] = ring.gen_range(0, 27) as u8;
                }
                random_tryte_array
            }

            pub fn encode(&self) -> $bname {
                let mut t_iter = self.0.iter();
                let mut encoded = $bname([B16(0, 0); $trit_size/9]);
                let mut b_iter_mut = encoded.0.iter_mut();
                for _ in 0..$trit_size/9 {
                    let t9 = t_iter.next().unwrap();
                    let b16 = b_iter_mut.next().unwrap();
                    *b16 = t9.encode();
                }
                encoded
            }

            pub fn from_human_readable(s : &str) -> Self {
                let mut tname = $tname([T9(0, 0, 0); $trit_size/9]);
                let mut src_iter = s.chars();
                let mut tname_iter_mut = tname.0.iter_mut();
                for _ in 0..(s.len()+2)/3 {
                    let c0 = src_iter.next().unwrap_or('9');
                    let c1 = src_iter.next().unwrap_or('9');
                    let c2 = src_iter.next().unwrap_or('9');
                    *tname_iter_mut.next().unwrap() = T9::from_human_readable(c0, c1, c2);
                }
                tname
            }

            pub fn from_tryte_array(array : [u8; $trit_size/3]) -> Self {
                let mut t9s = [T9(0, 0, 0); $trit_size/9];
                for i in 0 .. $trit_size/9 {
                    t9s[i] = T9(array[i*3], array[i*3+1], array[i*3+2]);
                }
                $tname(t9s)
            }

            pub fn as_tryte_array(&self) -> [u8; $trit_size/3] {
                let mut array = [0 as u8; $trit_size/3];
                let mut array_iter_mut = array.iter_mut();
                let mut t9_iter = self.0.iter();
                for _ in 0 .. $trit_size/9 {
                    let t9 = t9_iter.next().unwrap();
                    *array_iter_mut.next().unwrap() = t9.0;
                    *array_iter_mut.next().unwrap() = t9.1;
                    *array_iter_mut.next().unwrap() = t9.2;
                }
                array
            }

            #[cfg(test)]
            pub fn test_tryte_array() {
                let tryte_array_original = $tname::random_tryte_array();
                let tryte_array_result = $tname::from_tryte_array(tryte_array_original).as_tryte_array();
                assert_eq!(tryte_array_original[0..$trit_size/3], tryte_array_result[0..$trit_size/3]);
            }

            #[cfg(test)]
            pub fn test_from_human_readable() {
                let tname_original = $tname::from_human_readable("AB9C");
                let tname_result = tname_original.encode().decode();
                assert_eq!([1, 2, 0, 3, 0] as [u8; 5], tname_result.as_tryte_array()[0..5]);
            }

            #[cfg(test)]
            pub fn test_encode_decode() {
                let tname_original = $tname::random();
                let bname = tname_original.encode();
                let tname_decoded = bname.decode();
                assert_eq!(tname_original.as_tryte_array()[0..$trit_size/3], tname_decoded.as_tryte_array()[0..$trit_size/3]);
            }
        }

        pub struct $bname(pub [B16; $trit_size/9]);

        impl $bname {
            pub fn decode(&self) -> $tname {
                let mut b_iter = self.0.iter();
                let mut decoded = $tname([T9(0, 0, 0); $trit_size/9]);
                let mut t_iter_mut = decoded.0.iter_mut();
                for _ in 0..$trit_size/9 {
                    let b16 = b_iter.next().unwrap();
                    let t9 = t_iter_mut.next().unwrap();
                    *t9 = b16.decode();
                }
                decoded
            }
        }
    };
}

trinary_struct!(T721, 721, B1296);
trinary_struct!(T243, 243, B432);
trinary_struct!(T81, 81, B144);
trinary_struct!(T27, 27, B48);