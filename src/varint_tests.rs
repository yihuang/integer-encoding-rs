#[cfg(test)]
mod tests {
    use varint::VarInt;

    #[test]
    fn test_required_space() {
        assert_eq!((1 as u32).required_space(), 1);
        assert_eq!((128 as u32).required_space(), 2);
        assert_eq!((16384 as u32).required_space(), 3);
        assert_eq!((2097151 as u32).required_space(), 3);
        assert_eq!((2097152 as u32).required_space(), 4);
    }

    #[test]
    fn test_encode_u64() {
        assert_eq!((300 as u32).encode_var_vec(), vec![0b10101100, 0b00000010]);
    }

    #[test]
    fn test_identity_u64() {
        for i in 1 as u64..100 {
            assert_eq!(u64::decode_var_vec(&i.encode_var_vec()), (i,1));
        }
        for i in 16400 as u64..16500 {
            assert_eq!(u64::decode_var_vec(&i.encode_var_vec()), (i,3));
        }
    }

    #[test]
    fn test_encode_i64() {
        assert_eq!((150 as i64).encode_var_vec(), (300 as u32).encode_var_vec());
        assert_eq!((-150 as i64).encode_var_vec(),
                   (299 as u32).encode_var_vec());
        assert_eq!((-2147483648 as i64).encode_var_vec(),
                   (4294967295 as u64).encode_var_vec());
    }

    #[test]
    fn test_encode_i16() {
        assert_eq!((150 as i16).encode_var_vec(), (300 as u32).encode_var_vec());
        assert_eq!((-150 as i16).encode_var_vec(),
                   (299 as u32).encode_var_vec());
    }
}