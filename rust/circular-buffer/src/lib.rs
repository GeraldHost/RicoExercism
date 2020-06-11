pub struct CircularBuffer<T: Clone + Default> {
    buffer: Vec<T>,
    head: usize,
    tail: usize,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone + Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(T::default());
        }

        Self { buffer, head: 0, tail: 0, size: 0 }    
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
       if self.buffer.len() == self.size {
           Err(Error::FullBuffer)
       } else {
           self.buffer[self.head] = element;
           self.head = (self.head + 1) % self.buffer.len();
           self.size += 1;
           Ok(())
       }
    }

    pub fn read(&mut self) -> Result<T, Error> {
       if self.size == 0 {
           Err(Error::EmptyBuffer)
       } else {
           let value = self.buffer[self.tail].clone();
           self.tail = (self.tail + 1) % self.buffer.len();
           self.size -= 1;
           Ok(value)
       } 
    }

    pub fn clear(&mut self) {
        if self.size != 0 {
            self.size = 0;
            let capacity = self.buffer.len();
            self.buffer.clear();
            for _ in 0..capacity {
                self.buffer.push(T::default());
            }
        }
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buffer.len() != self.size {
            self.write(element);
        } else {
            self.buffer[self.tail] = element;
            self.tail += 1;
        }
    }
}
