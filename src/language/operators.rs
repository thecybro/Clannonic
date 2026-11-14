
use super::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignOp {
    Assign, // =
    AddAssign, // +=
    SubtractAssign, // -=
}

impl AssignOp {
    pub fn validates(
        self,
        left: Type,
        right: Type,
    ) -> bool {
        match self {
            AssignOp::AddAssign
            | AssignOp::SubtractAssign => 
            {
                let arith_op = if self == AssignOp::AddAssign {
                    ArithmeticOp::Add
                } else {
                    ArithmeticOp::Subtract
                };

                // Arithmetic operator owns arithmetic operations; so we
                // convert the assignment operator to the corresponding
                // arithmetic operator and let it tell whether or not
                // the operation is valid
                if let Some(_) = arith_op.result_type(left, right) {
                    true
                } else {
                    false
                }
            }

            AssignOp::Assign 
            // When assigning, all we care about is left should be the same as right,
            // and we dont care whether left is int, bool or whatever
                if left == right => 
            {
                true
            }

            _ => false,
        }
    }
}

/// The operators you can use to compare stuff
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ComparisonOp {
    IsEqualTo,
    IsNotEqualTo,
    IsGreaterThan,
    IsLessThan,
    IsGreaterOrEqualTo,
    IsLessOrEqualTo,
}

impl ComparisonOp {
    pub fn return_type(
        self,
        left: Type,
        right: Type,
    ) -> Option<Type> {
        match self {
            ComparisonOp::IsEqualTo
            | ComparisonOp::IsNotEqualTo
            | ComparisonOp::IsGreaterThan
            | ComparisonOp::IsLessThan
            | ComparisonOp::IsGreaterOrEqualTo
            | ComparisonOp::IsLessOrEqualTo
                if left == Type::Int && right == Type::Int =>
            {
                Some(Type::Bool)
            }

            _ => None,
        }
    }
}

/// The binary operators for arithmetic operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArithmeticOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    LogicalAnd, // &&
    LogicalOr, // ||
}

impl ArithmeticOp {
    pub fn result_type(
        self,
        left: Type,
        right: Type
    ) -> Option<Type> {
        match self {
            ArithmeticOp::Add
            | ArithmeticOp::Subtract
            | ArithmeticOp::Multiply
            | ArithmeticOp::Divide
                if left == Type::Int && right == Type::Int =>
                {
                    Some(Type::Int)
                }

            ArithmeticOp::LogicalAnd
            | ArithmeticOp::LogicalOr
                if left == Type::Bool && right == Type::Bool => 
                {
                    Some(Type::Bool)
                }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UnaryOp {
    Plus, // +x
    Minus, // -x
    Increment, // x++
    Decrement, // x--
    Not, // !x
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryFixity {
    Prefix,
    Postfix,
}

impl UnaryOp {
    pub fn return_type(
        self,
        operand: Type,
        fixity: UnaryFixity
    ) -> Option<Type> {
        match fixity {
            UnaryFixity::Postfix => self.ret_ty_postfix(operand),
            UnaryFixity::Prefix => self.ret_ty_prefix(operand)
        }
    }

    fn ret_ty_postfix(
        self,
        operand: Type,
    ) -> Option<Type> {
        match self {
            UnaryOp::Increment
            | UnaryOp::Decrement
                if operand == Type::Int =>
            {
                Some(Type::Int)
            }

            _ => None,
        }
    }
    
    fn ret_ty_prefix(
        self,
        operand: Type,
    ) -> Option<Type> {
        match self {
                UnaryOp::Plus
                    if operand == Type::Int => 
                {
                    Some(Type::Int)
                }

                UnaryOp::Minus
                    if operand == Type::Int =>
                {
                    Some(Type::Int)
                }

                UnaryOp::Not
                    if operand == Type::Bool =>
                {
                    Some(Type::Bool)
                }

                _ => None
            }
    }
}