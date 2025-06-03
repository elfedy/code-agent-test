struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        match self.next.checked_add(self.current) {
            Some(next) => {
                self.current = self.next;
                self.next = next;
                Some(current)
            }
            None => None,
        }
    }
}
