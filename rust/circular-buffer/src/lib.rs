pub struct CircularBuffer<T>
where
    T: Default,
{
    buffer: Vec<Option<T>>,
    write_pos: usize,
    read_pos: usize,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: std::default::Default + std::clone::Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: CircularBuffer::init(capacity),
            write_pos: 0,
            read_pos: 0,
            capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if let Some(_v) = self.buffer[self.write_pos].take() {
            Err(Error::FullBuffer)
        } else {
            self.buffer[self.write_pos].replace(element);
            self.write_pos += 1;
            self.write_pos %= self.capacity;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Some(v) = self.buffer[self.read_pos].take() {
            self.read_pos += 1;
            self.read_pos %= self.capacity;
            Ok(v)
        } else {
            Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        self.write_pos = 0;
        self.read_pos = 0;
        self.buffer = Self::init(self.capacity);
    }

    pub fn overwrite(&mut self, element: T) {
        if self.read_pos == self.write_pos {
            self.read_pos += 1;
            self.read_pos %= self.capacity;
        }
        self.buffer[self.write_pos].replace(element);
        self.write_pos += 1;
        self.write_pos %= self.capacity;
    }

    fn init(capacity: usize) -> Vec<Option<T>> {
        let mut v = Vec::with_capacity(capacity);
        v.resize(capacity, None);
        v
    }
}
