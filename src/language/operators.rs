
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign, // =
    AddAssign, // +=
    SubtractAssign, // -=
}


/// The operators you can use to compare stuff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConditionalOp {
    IsEqualTo,
    IsNotEqualTo,
    IsGreaterThan,
    IsLessThan,
    IsGreaterOrEqualTo,
    IsLessOrEqualTo,
}


/// The binary operators for arithmetic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    LogicalAnd, // &&
    LogicalOr, // ||
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnaryOp {
    Plus,
    Minus,
    Increment,
    Decrement,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryFixity {
    Prefix,
    Postfix,
}

