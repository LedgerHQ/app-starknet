use serde::Serialize;
use std::fmt;

const MAX_APDU_DATA_SIZE: usize = 255;

#[derive(Default, Clone, Copy, Serialize)]
pub struct ApduHeader {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
}

#[derive(Default, Clone, Serialize)]
pub struct Apdu {
    pub header: ApduHeader,
    #[serde(with = "hex::serde")]
    pub data: Vec<u8>,
}

impl Apdu {
    pub fn new(header: ApduHeader) -> Self {
        Apdu {
            header,
            data: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    pub fn append(&mut self, data: &[u8]) -> Result<(), usize> {
        if self.data.len() + data.len() <= MAX_APDU_DATA_SIZE {
            self.data.extend_from_slice(data);
            Ok(())
        } else {
            Err(MAX_APDU_DATA_SIZE - self.data.len())
        }
    }
}

impl fmt::Display for Apdu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02x}", self.header.cla)?;
        let ins: u8 = self.header.ins;
        write!(f, "{:02x}", ins)?;
        write!(f, "{:02x}", self.header.p1)?;
        write!(f, "{:02x}", self.header.p2)?;
        write!(f, "{:02x}", self.data.len())?;
        for b in &self.data {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
