pub trait Round: Copy {
    /// Rounds to largest whole number that is less or equal.
    fn floor(self) -> Self;

    /// Rounds to largest whole number that is greater or equal.
    fn ceil(self) -> Self;

    /// Rounds to the nearest whole number with half-way cases rounding away from zero.
    fn round(self) -> Self;

    /// Rounds to the nearest whole number with half-way cases rounding to even.
    fn round_ties_even(self) -> Self;
}

impl Round for f32 {
    fn floor(self) -> Self {
        self.floor()
    }

    fn ceil(self) -> Self {
        self.ceil()
    }

    fn round(self) -> Self {
        self.round()
    }

    fn round_ties_even(self) -> Self {
        self.round_ties_even()
    }
}

impl Round for f64 {
    fn floor(self) -> Self {
        self.floor()
    }

    fn ceil(self) -> Self {
        self.ceil()
    }

    fn round(self) -> Self {
        self.round()
    }

    fn round_ties_even(self) -> Self {
        self.round_ties_even()
    }
}
