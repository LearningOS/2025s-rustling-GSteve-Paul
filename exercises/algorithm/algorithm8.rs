/*
    queue
    This question requires you to use queues to implement the functionality of the stac
*/
#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T> {
    //TODO
    now: usize,
    q1: Queue<T>,
    q2: Queue<T>,
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            //TODO
            now: 1,
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }
    pub fn push(&mut self, elem: T) {
        let qnow = {
            if self.now == 1 {
                &mut self.q1
            } else {
                &mut self.q2
            }
        };
        qnow.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        let qnow: &mut Queue<T>;
        let qother: &mut Queue<T>;

        if self.now == 1 {
            qnow = &mut self.q1;
            qother = &mut self.q2;
        } else {
            qnow = &mut self.q2;
            qother = &mut self.q1;
        }

        while !qother.is_empty() {
            qother.dequeue();
        }
        while qnow.size() > 1 {
            let elem = qnow.dequeue().unwrap();
            qother.enqueue(elem);
        }
        if self.now == 1 {
            self.now = 2;
        } else {
            self.now = 1;
        }
        qnow.dequeue().or(Err("Stack is empty"))
    }
    pub fn is_empty(&self) -> bool {
        let qnow = {
            if self.now == 1 {
                &self.q1
            } else {
                &self.q2
            }
        };
        qnow.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}
