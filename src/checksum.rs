#[cfg(test)]
mod tests;

/// A internet checksum calculator based on a builder pattern.
pub struct Checksum {
    value: u16
}

impl Checksum {
    pub fn new() -> Checksum {
        Checksum {
            value: 0
        }
    }

    pub fn add_data(&mut self, data: u16) -> &mut Checksum {
        let (s, c) = self.value.overflowing_add(data);
        self.value = s;

        if c {
            // We don't need to check overflow here because the highest value
            // if the previous calculation overflowed would be max-2 and the
            // number can go to max-1
            self.value += 1;
        }

        self
    }

    pub fn add_all_data(&mut self, data: &[u16]) -> &mut Checksum {
        for d in data {
            self.add_data(*d);
        }

        self
    }

    pub fn checksum(&self) -> u16 {
        !self.value
    }
}
