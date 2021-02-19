#[cfg(test)]
mod tests {
    use crate::util::calculate_crc;
    #[test]
    fn test_crc_len1() {
        assert_eq!(calculate_crc(&vec![0x0]), 0x100);
        assert_eq!(calculate_crc(&vec![0x1]), 0x201);
        assert_eq!(calculate_crc(&vec![0x2]), 0x302);
        assert_eq!(calculate_crc(&vec![0x3]), 0x403);
    }
    #[test]
    fn test_crc_len2() {
        assert_eq!(calculate_crc(&vec![0x0, 0x0]), 0x4200);
        assert_eq!(calculate_crc(&vec![0x1, 0x1]), 0x8441);
        assert_eq!(calculate_crc(&vec![0x4, 0x1]), 0x14501);
    }
    #[test]
    fn test_crc_len3() {
        assert_eq!(calculate_crc(&vec![0x0, 0x0, 0x0]), 0x108300);
    }
    #[test]
    fn test_crc_len5() {
        assert_eq!(calculate_crc(&vec![0x1, 0x2, 0x3, 0x4, 0x5]), 0x19CD4F07);
    }
}

pub fn calculate_crc(data: &[u8]) -> u32 {
    let mut register: u32 = 0;
    let mut index: usize = 0;
    while index != data.len() {
        register = register.rotate_left(6);
        let dat: u32 = data[index] as u32;
        index += 1;
        register = (std::num::Wrapping(register ^ dat) + std::num::Wrapping(((dat + 1)) * (index as u32) << 8)).0;
    }
    return register;
}
