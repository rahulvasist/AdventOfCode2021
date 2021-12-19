aoc_main::main! {
    year 2021;
    day16 : generator => part_1, part_2;
}

mod day16 {
    use itertools::Itertools;

    pub struct Packet<'a> {
        data: &'a [u8],
        cursor: usize,
    }

    impl<'a> Packet<'a> {
        pub fn new(data: &'a [u8]) -> Self {
            Self { data, cursor: 0 }
        }

        pub fn get_bits(&mut self, n: usize) -> u32 {
            assert!(n <= 16);
            let byte = self.cursor / 8;
            let end_byte = (self.cursor + n) / 8;
            let offset = self.cursor % 8;
            match end_byte - byte {
                0 => {
                    let val = u8::from_be_bytes(self.data[byte..byte + 1].try_into().unwrap());
                    let start = 7 - offset;
                    let end = start - n + 1;
                    let mask = (1 << n) - 1;
                    self.cursor += n;
                    ((val >> end) & mask).into()
                }
                1 => {
                    let val = u16::from_be_bytes(self.data[byte..byte + 2].try_into().unwrap());
                    let start = 15 - offset;
                    let end = start - n + 1;
                    let mask = (1 << n) - 1;
                    self.cursor += n;
                    ((val >> end) & mask).into()
                }
                2 => {
                    let val1 =
                        u16::from_be_bytes(self.data[byte..byte + 2].try_into().unwrap()) as u32;
                    let val2 =
                        u8::from_be_bytes(self.data[byte + 2..byte + 3].try_into().unwrap()) as u32;
                    let val: u32 = val1 << 8 | val2;
                    let start = 23 - offset;
                    let end = start - n + 1;
                    let mask = (1 << n) - 1;
                    self.cursor += n;
                    ((val >> end) & mask).into()
                }
                _ => unreachable!(),
            }
        }
    }

    const TYPE_ID_LITERAL: u32 = 4;

    pub fn parse_packet(p: &mut Packet) -> (usize, usize) {
        // let mut p = Packet::new(data);

        println!("********* Start *********************");
        let mut version_sum: usize = 0;
        let version = dbg!(p.get_bits(3)) as usize;
        let type_id = dbg!(p.get_bits(3));
        version_sum += version;

        let value: usize = match type_id {
            TYPE_ID_LITERAL => {
                let mut n = 0;
                loop {
                    let last_num = dbg!(p.get_bits(1) == 0);
                    let number = dbg!(p.get_bits(4)) as usize;
                    n = (n << 4) | number;

                    if last_num {
                        break;
                    }
                }
                n
            }
            _ => {
                let len_type_id = dbg!(p.get_bits(1));
                let mut sub_packets = Vec::new();
                if len_type_id == 0 {
                    let len_sub_packets: usize = dbg!(p.get_bits(15)) as usize;
                    let prev_cursor = dbg!(p.cursor);
                    loop {
                        let (v, e) = dbg!(parse_packet(p));
                        version_sum += v;
                        sub_packets.push(e);
                        if dbg!(p.cursor - prev_cursor) >= dbg!(len_sub_packets) {
                            break;
                        }
                    }
                } else {
                    let num_sub_packets = dbg!(p.get_bits(11));
                    for _ in 0..num_sub_packets {
                        let (v, e) = dbg!(parse_packet(p));
                        version_sum += v;
                        sub_packets.push(e);
                    }
                }
                match type_id {
                    0 => sub_packets.into_iter().sum(),
                    1 => sub_packets.into_iter().product(),
                    2 => sub_packets.into_iter().min().unwrap(),
                    3 => sub_packets.into_iter().max().unwrap(),
                    5 => {
                        if sub_packets[0] > sub_packets[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if sub_packets[0] < sub_packets[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if sub_packets[0] == sub_packets[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                }
            }
        };
        println!("********* end *********************");
        (version_sum, value)
    }

    pub fn generator(input: &str) -> Vec<u8> {
        let mut vec = Vec::new();
        for i in 0..input.len() / 2 {
            let s = &input[i * 2..(i * 2 + 2)];
            vec.push(u8::from_str_radix(s, 16).unwrap());
        }
        vec
    }

    pub fn part_1(input: &Vec<u8>) -> usize {
        let mut p = Packet::new(input.as_slice());
        parse_packet(&mut p).0
    }

    pub fn part_2(input: &Vec<u8>) -> usize {
        let mut p = Packet::new(input.as_slice());
        parse_packet(&mut p).1
    }
}

#[cfg(test)]
mod tests {
    use super::day16::*;

    #[test]
    fn test_basic() {
        let data = [0xd2, 0xfe, 0x28];
        let mut p = Packet::new(&data[..]);
        assert_eq!(6, p.get_bits(3));
        assert_eq!(4, p.get_bits(3));
        assert_eq!(5, p.get_bits(3));
        assert_eq!(31, p.get_bits(5));
        assert_eq!(2, p.get_bits(2));
    }

    #[test]
    fn test_1() {
        let s = "FAAB";
        assert_eq!(vec![0xfa, 0xab], generator(s));
    }

    #[test]
    fn test_2() {
        let input = generator("8A004A801A8002F478");
        let mut p = Packet::new(input.as_slice());
        assert_eq!(16, parse_packet(&mut p).0);

        let input = generator("620080001611562C8802118E34");
        let mut p = Packet::new(input.as_slice());
        assert_eq!(12, parse_packet(&mut p).0);

        let input = generator("C0015000016115A2E0802F182340");
        let mut p = Packet::new(input.as_slice());
        assert_eq!(23, parse_packet(&mut p).0);

        let input = generator("A0016C880162017C3686B18A3D4780");
        let mut p = Packet::new(input.as_slice());
        assert_eq!(31, parse_packet(&mut p).0);
    }

    #[test]
    fn test_3() {
        let input = generator("04005AC33890");
        let mut p = Packet::new(input.as_slice());
        assert_eq!(54, parse_packet(&mut p).1);

        let input = generator("880086C3E88112");
        let mut p = Packet::new(input.as_slice());
        assert_eq!(7, parse_packet(&mut p).1);

        let input = generator("CE00C43D881120");
        let mut p = Packet::new(input.as_slice());
        assert_eq!(9, parse_packet(&mut p).1);

        let input = generator("9C0141080250320F1802104A08");
        let mut p = Packet::new(input.as_slice());
        assert_eq!(1, parse_packet(&mut p).1);
    }
}
