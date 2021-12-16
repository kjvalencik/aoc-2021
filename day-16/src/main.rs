use util::*;

#[derive(Debug, Eq, PartialEq)]
struct Puzzle {
    packet: Packet,
}

#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: u8,
    data: Data,
}

impl Packet {
    fn operate(&self) -> u64 {
        match &self.data {
            Data::Literal(n) => *n,
            Data::Operator(op) => op.exec(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Data {
    Literal(u64),
    Operator(Operator),
}

#[derive(Debug, Eq, PartialEq)]
struct Operator {
    op: OperatorType,
    packets: Vec<Packet>,
}

impl Operator {
    fn new(op: u8, packets: Vec<Packet>) -> Result<Self, Error> {
        let op = match op {
            0 => OperatorType::Sum,
            1 => OperatorType::Product,
            2 => {
                if packets.is_empty() {
                    bail!("Minimum requires at least 1 packet");
                }
                OperatorType::Minimum
            }
            3 => {
                if packets.is_empty() {
                    bail!("Maximum requires at least 1 packet");
                }
                OperatorType::Maximum
            }
            5 => {
                if packets.len() != 2 {
                    bail!("GreaterThan requires exactly 2 packets");
                }
                OperatorType::GreaterThan
            }
            6 => {
                if packets.len() != 2 {
                    bail!("LessThan requires exactly 2 packets");
                }
                OperatorType::LessThan
            }
            7 => {
                if packets.len() != 2 {
                    bail!("EqualTo requires exactly 2 packets");
                }
                OperatorType::EqualTo
            }
            _ => bail!("Invalid operator type"),
        };

        Ok(Self { op, packets })
    }
}

impl Operator {
    // Unwrap are safe because data has already been validated
    fn exec(&self) -> u64 {
        let mut data = self.packets.iter().map(Packet::operate);

        match self.op {
            OperatorType::Sum => data.sum(),
            OperatorType::Product => data.product(),
            OperatorType::Minimum => data.min().unwrap(),
            OperatorType::Maximum => data.max().unwrap(),
            OperatorType::GreaterThan => {
                if data.next().unwrap() > data.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            OperatorType::LessThan => {
                if data.next().unwrap() < data.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            OperatorType::EqualTo => {
                if data.next().unwrap() == data.next().unwrap() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[derive(Debug)]
enum Length {
    Bits(usize),
    Packets(usize),
}

#[derive(Debug)]
struct Reader<'a> {
    buf: &'a [u8],
}

impl<'a> Reader<'a> {
    fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }

    fn next(&mut self) -> Result<Option<Packet>, Error> {
        if self.buf.is_empty() {
            return Ok(None);
        }

        let version = self.version()?;
        let data = match self.label()? {
            4 => Data::Literal(self.literal()?),
            id => Data::Operator(self.operator(id)?),
        };

        Ok(Some(Packet { version, data }))
    }

    fn literal(&mut self) -> Result<u64, Error> {
        let mut chunk = self.take(5)?;
        let mut data = chunk[1..].to_vec();

        while chunk[0] == b'1' {
            chunk = self.take(5)?;
            data.extend_from_slice(&chunk[1..]);
        }

        Ok(u64::from_str_radix(str::from_utf8(&data)?, 2)?)
    }

    fn operator(&mut self, op: u8) -> Result<Operator, Error> {
        let packets = match self.length()? {
            Length::Bits(len) => {
                let mut reader = Reader::new(self.take(len)?);
                let mut packets = Vec::new();

                while let Some(packet) = reader.next()? {
                    packets.push(packet);
                }

                packets
            }
            Length::Packets(len) => {
                let mut packets = Vec::with_capacity(len);

                for _ in 0..len {
                    packets.push(self.next()?.context("Missing packet")?);
                }

                packets
            }
        };

        Operator::new(op, packets)
    }

    fn length(&mut self) -> Result<Length, Error> {
        let parse =
            |buf| -> Result<_, Error> { Ok(usize::from_str_radix(str::from_utf8(buf)?, 2)?) };

        Ok(match self.take(1)?[0] {
            b'0' => Length::Bits(parse(self.take(15)?)?),
            b'1' => Length::Packets(parse(self.take(11)?)?),
            _ => bail!("Invalid length"),
        })
    }

    fn label(&mut self) -> Result<u8, Error> {
        self.number(3)
    }

    fn version(&mut self) -> Result<u8, Error> {
        self.number(3)
    }

    fn number(&mut self, n: usize) -> Result<u8, Error> {
        Ok(u8::from_str_radix(str::from_utf8(self.take(n)?)?, 2)?)
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        if self.buf.len() < n {
            self.buf = &[];
            bail!("Insufficient buffer");
        }
        let output = &self.buf[0..n];
        self.buf = &self.buf[n..];
        Ok(output)
    }
}

fn decode_hex(input: &str) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::with_capacity(4 * input.len());

    for c in input.trim().chars() {
        buf.extend_from_slice(match c {
            '0' => b"0000",
            '1' => b"0001",
            '2' => b"0010",
            '3' => b"0011",
            '4' => b"0100",
            '5' => b"0101",
            '6' => b"0110",
            '7' => b"0111",
            '8' => b"1000",
            '9' => b"1001",
            'A' => b"1010",
            'B' => b"1011",
            'C' => b"1100",
            'D' => b"1101",
            'E' => b"1110",
            'F' => b"1111",
            _ => bail!("Invalid character"),
        });
    }

    Ok(buf)
}

impl FromStr for Puzzle {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let buf = decode_hex(s)?;
        let mut reader = Reader::new(&buf);
        let packet = reader.next()?.context("Missing packet")?;

        Ok(Self { packet })
    }
}

impl Puzzle {
    fn part_1(&self) -> u64 {
        fn sum(total: u64, packet: &Packet) -> u64 {
            let total = total + u64::from(packet.version);
            let operator = match &packet.data {
                Data::Operator(o) => o,
                _ => return total,
            };

            let sum = operator
                .packets
                .iter()
                .map(|packet| sum(0, packet))
                .sum::<u64>();

            sum + total
        }

        sum(0, &self.packet)
    }

    fn part_2(&self) -> u64 {
        self.packet.operate()
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::from_str(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1());
    println!("Part 1: {}", puzzle.part_2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn literal() -> Result<(), Error> {
        assert_eq!(
            Puzzle::from_str("D2FE28")?,
            Puzzle {
                packet: Packet {
                    version: 6,
                    data: Data::Literal(2021),
                }
            }
        );

        Ok(())
    }

    #[test]
    fn operator() -> Result<(), Error> {
        assert_eq!(
            Puzzle::from_str("38006F45291200")?,
            Puzzle {
                packet: Packet {
                    version: 1,
                    data: Data::Operator(Operator {
                        op: OperatorType::LessThan,
                        packets: vec![
                            Packet {
                                version: 6,
                                data: Data::Literal(10),
                            },
                            Packet {
                                version: 2,
                                data: Data::Literal(20),
                            }
                        ]
                    }),
                }
            }
        );

        Ok(())
    }

    #[test]
    fn part_1() -> Result<(), Error> {
        assert_eq!(Puzzle::from_str("8A004A801A8002F478")?.part_1(), 16);
        assert_eq!(Puzzle::from_str("620080001611562C8802118E34")?.part_1(), 12);
        assert_eq!(
            Puzzle::from_str("C0015000016115A2E0802F182340")?.part_1(),
            23
        );
        assert_eq!(
            Puzzle::from_str("A0016C880162017C3686B18A3D4780")?.part_1(),
            31
        );

        Ok(())
    }
}
