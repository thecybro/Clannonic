# LANGUAGE SPECIFICATION

> In this file, you would find what exists in the language aka the specification

> [!Note]
> Note that the things defined here are the things we have decided to design/stabilize first
> before adding more; so even if you see support for other features/types, note that they
> were part of the early experimentation and could be removed later; to let the basics of the
> language stabilize first

## 1. Scope
> The core language provides:
- variables
- integer & boolean values
- arithmetic
- comparisons
- functions
- conditional execution
- loops
- return values
- lexical scopes
- static type checking

## 2. Types
`Int`,
`Bool`,
`Unit` (No type/meaningful value)

For example:
```
// returns int
fn main() -> int {
  return 1;
}

// returns unit type
fn main() {
  
}
```

## 3. Variables
Variable declarations we aim for:
```
let x: int = 10; // immutable binding
// Or it can be written this way as well
let x = 10; // int and immutable

x = 20; // Invalid

let mut y: int = 20; // mutable binding
y = 30; // valid
```

## 4. Statements
Let
Assignment
If
While
Return
Function declaration
Expression statement

Examples
```
let mut x: int = 10;
x = x + 1; // or x +=1;

if x > 0 {
  x = x - 1;
}

while x > 0 {
  x = x - 1;
}

print(x)
```

## 5. Expressions
Integer literals
Boolean literals
Identifiers
Parenthesized expressions
Binary operators
Unary operators
Comparisons
Function calls

Examples
```
10
true
x
(1 + 2) * 3
-x
!enabled
add(1, 2)
```

## 6. Operators

### Arithmetic
`+` addition
`-` subtraction
`*` multiplication
`/` division

#### Rules
Int + Int -> Int
Int - Int -> Int
Int * Int -> Int
Int / Int -> Int

For example:
```
1 + 2.0 // This will not work 
```

### Comparison
`==` equality
`!=` inequality
`<` less than
`<=` less than or equal
`>` greater than
`>=` greater than or equal

#### Rules
Int == Int -> Bool
Int < Int  -> Bool
Bool == Bool -> Bool
Bool != Bool -> Bool

### Boolean operators
`!` logical not
`&&` logical and
`||` logical or

#### Rules
!Bool -> Bool
Bool && Bool -> Bool
Bool || Bool -> Bool

> `&& and ||` short circuit.
> Meaning `false && do_something()`, here do_something() is not evaluated
> `false || true` evaluates to `true` but `false || false` evaluates to false

### Assignment Operators
`=` normal assign
`+=` add then assign
`-=` subtract then assign

Examples
`a += 1` is the same as `a = a + 1`
`a -= 1` is the same as `a = a - 1`

But only if they were declared mutable

## 7> Operator precendence
From lowest to highest
- assiignment, statement
- `||`
- `&&`
- `==` `!=`
- `<` `<=` `>` `>=`
- `+` `-`
- `*` `/`
- prefix unary: `!` `-` `+`
- postfix operations, function calls
- primary: literals, identifiers, parentheses

Example
`1 + 2 * 3` means `1 + (2 * 3)`
`-x * 2` means `(-x) * 2`

## 8. Functions
Typed parameters
One return type

Examples
- Function declarations
```
fn add(x: int, y: int) -> int {
  return x + y;
}
```

- Function calls
```
let result = add(1, 2);
```

#### Rules
- Functions have typed parameters
- calls must provide the correct number of arguments
- arguments must have compatible types
- return expressions must match teh declared return type
- functions without a declared return type return `Unit`

Example
```
fn log(x: int) {
  // no return value
}
```

## 9. Control flow
- `if`
```
if condition {
  // statements
}
```
> The condition must result in the type `bool`
```
if x > 10 {
  return x;
}
```
And else after the basic version works
```
if condition {
  
} else {

}
```

- `while`
```
while condition {
  // statements
}
```
> The condition must have the type `bool`.