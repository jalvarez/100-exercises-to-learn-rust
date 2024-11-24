// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.
pub fn fibonacci(n: u32) -> u32 {
    // TODO: implement the `fibonacci` function
    //
    // Hint: use a `Vec` to memoize the results you have already calculated
    // so that you don't have to recalculate them several times.
    let mut fm = FiboMem::new();
    fm.fibonacci(n)
}

struct FiboMem {
    previous_results: Vec<u32>,
}

impl FiboMem {
    pub fn new() -> Self {
        FiboMem {
            previous_results: vec![0, 1],
        }
    }

    fn fibonacci(&mut self, n: u32) -> u32 {
        let pos: usize = n.try_into().unwrap();
        match self.previous_results.get(pos) {
            Some(r) => *r,
            None => {
                let result = self.fibonacci(n - 2) + self.fibonacci(n - 1);
                self.previous_results.push(result);
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
