pub trait IsEven {
    fn is_even(self) -> bool;
}

impl IsEven for i32 {
    fn is_even(self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for usize {
    fn is_even(self) -> bool {
        self % 2 == 0
    }
}

// Generic function that works for any type that implements IsEven
pub fn is_even<T: IsEven>(num: T) -> bool {
    num.is_even()
}
