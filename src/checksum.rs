#[cfg(test)]
mod tests;

/// A internet checksum calculator based on a builder pattern.
///
/// # Examples
///
/// ```
/// use pokey_checksum::checksum::Checksum;
///
/// assert_eq!(
///     0b0100010001000011,
///     Checksum::new()
///         .add_data(0b1110011001100110)
///         .add_data(0b1101010101010101)
///         .checksum()
/// );
/// ```
pub struct Checksum {
    value: u16
}

impl Checksum {
    /// Initialize a new checksum builder/calculator.
    ///
    /// Usually should either be used inline or as a mutable variable.
    pub fn new() -> Checksum {
        Checksum {
            value: 0
        }
    }

    /// Add a 16-bit word to the checksum.
    ///
    /// Returns a mutable reference so that method calls can be either chained,
    /// or separate.
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

    /// Add an entire array of 16-bit words to the checksum.
    ///
    /// This method is equivalent to just iterating and calling add_data for
    /// each element of the array.
    pub fn add_all_data(&mut self, data: &[u16]) -> &mut Checksum {
        for d in data {
            self.add_data(*d);
        }

        self
    }

    /// Calculate the final checksum based on the data provided.
    ///
    /// This is essentially the flipped bits of the one's compliment sum of all
    /// the data points up to this point.
    pub fn checksum(&self) -> u16 {
        // The internet checksum flips the bits.
        !self.value
    }
}
