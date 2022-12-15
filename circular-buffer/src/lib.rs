pub struct CircularBuffer<T> {
    cursor: usize,
    capacity: usize,
    data: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            cursor: 0usize,
            capacity,
            data: vec![],
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.data.len() == self.capacity {
            return Err(Error::FullBuffer);
        }

        let mut cursor = self.cursor;
        while self.data.get(cursor).is_some() {
            cursor = self.next_cursor();
        }

        self.data.insert(cursor, element);
        self.next_cursor();
        Ok(())
    }

    fn next_cursor(&mut self) -> usize {
        self.cursor = (self.cursor + 1) % self.capacity;
        self.cursor
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.data.len() == 0 {
            return Err(Error::EmptyBuffer);
        }

        let mut cursor = self.cursor;
        while self.data.get(cursor).is_none() {
            cursor = self.next_cursor();
        }

        let data = self.data.remove(cursor);
        Ok(data)
    }

    pub fn clear(&mut self) {
        self.data = vec![]
    }

    pub fn overwrite(&mut self, element: T) {
        let cursor = self.cursor;
        if self.data.get(cursor).is_some() {
            self.data.remove(cursor);
        }
        self.data.insert(cursor, element);
        self.next_cursor();
    }
}
