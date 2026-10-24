# The example syntax 

> Here you would find some example code you can checkout to get going with this language

Currently, the language is very simple and intuitive.

It also doesn't have concept of any types or scopes yet

Here are the example ways you can use it:
> [!NOTE]
> These are examples of the valid syntax as of October 24 2026 and may/may not change later
>
## Examples
> [!NOTE]
> The limitation as of October 24, 2026 is that even once you initialize a variable
> by doing `let var = `, if you want to change its value, you gotta do `let var = ` again
> because the language only understands the statements starting with the keywords 
> (`let`, `if`, `while`, `fn` etc for now)

### Initialize a variable
```
let x = 1
let y = 2
let z = "a_string"

let a = main() // store the return value of main function in 'a'
```

### Add 2 numbers
```
let z = 1 + 2
let y = z + 1
```

### If statement
> [!NOTE]
> No functions implemented yet to atleast print something, so until then, you would see
> the message of what is stored in what variable in the terminal
```
if a > b {
  let x = a
}
```

You can also put other statements inside if, but elif/else if aren't supported yet
```
let x = 1
if x < 2 {
  let x = 3
  if x > 2 {
    let x = 1
  }
}
```

### While statement/loop
> While also supports other statements inside it.
```
let x = 10
while x > 2 {
  let x = x -1
}
```

### Creating a function
> This doesn't support arguments yet
>
> [!NOTE]
> Here, the type indication (-> type) of the return type is compulsary, so you can't
> just do `fn main() {..}`
```
fn main() -> String {
  let x = "smt"

  return x
}
```

```
fn sum_of_five_nums() -> int {
  let x = 1
  let sum = 0

  while x <= 5 {
    let sum = sum + x

    let x = x + 1
  }

  return sum
}

// Now sum stores the sum
let sum = sum_of_five_nums() 

```