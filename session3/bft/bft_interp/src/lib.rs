pub struct BfTape<T> {
    cell: Vec<T>,
    grow: bool,
}

impl<T> BfTape<T> {
    // Standard method of new
    // pub fn new(&mut self, size: usize) -> &BfTape<T> {
    //     if size == 0 {
    //         self.cell = Vec::<T>::with_capacity(30000);
    //     } else {
    //         self.cell = Vec::<T>::with_capacity(size);
    //     }
    //     return self;
    // }

    // Spicy method of new
    pub fn new(&mut self, size: Option<core::num::NonZeroUsize>, grow: bool) -> &BfTape<T> {
        self.grow = grow;
        match size {
            None => self.cell = Vec::<T>::with_capacity(30000),
            Some(s) => self.cell = Vec::<T>::with_capacity(s.into()),
        }
        return self;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
