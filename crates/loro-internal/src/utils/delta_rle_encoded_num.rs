use loro_common::Lamport;
use serde_columnar::columnar;

#[columnar(vec, ser, de)]
#[derive(Debug, Clone)]
struct EncodedNum {
    #[columnar(strategy = "DeltaRle")]
    num: Lamport,
}

#[derive(Default)]
#[columnar(ser, de)]
pub struct DeltaRleEncodedNums {
    #[columnar(class = "vec")]
    nums: Vec<EncodedNum>,
}

impl DeltaRleEncodedNums {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, n: Lamport) {
        self.nums.push(EncodedNum { num: n });
    }

    pub fn iter(&self) -> impl Iterator<Item = Lamport> + '_ {
        self.nums.iter().map(|n| n.num)
    }

    pub fn encode(&self) -> Vec<u8> {
        serde_columnar::to_vec(&self).unwrap()
    }

    pub fn decode(encoded: &[u8]) -> Self {
        serde_columnar::from_bytes(encoded).unwrap()
    }
}
