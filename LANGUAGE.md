# LANGUAGE SPECIFICATIONS

> Here, you would find:
>
> The syntax, types, scopes, expressions, statements, and runtime behaviour of **Clannonic**.

This file is to describe what a program means, not how it's implemented.

Currently, the language is very simple and intuitive.

It also doesn't have concept of any types or scopes yet

Here are the example ways you can use it:
> Note that as of October 13, 2026, I haven't written the evaluater
or code generator yet, so you can't actually use the language, but you can observe the tokens it produces and
AST it generates.
>
> I have written these examples assuming if there was an evaluater/code generator now

## Examples
> [!NOTE]
> The limitation as of October 13, 2026 is that even once you initialize a variable
> by doing `let var = `, if you want to change its value, you gotta do `let var = ` again
> because the language only understands the statements starting with the keywords 
> (`let`, `if`, `while` etc for now)

### Initialize a variable
> [!NOTE]
> As of October 13, 2026, you can't yet initialize a variable with strings,
by doing `let x = "whatever"` because even though it will
correctly tokenize it, the parsing ability
for it isn't there yet, I will add it soon
```
let x = 1
let y = 2
let z = "a_string"
```

### Add 2 numbers
```
let z = 1 + 2
```

### If statement
> [!NOTE]
> No functions implemented yet to atleast print something
```
if a > b {
  let x = a
}
```

You can also put other statements inside if, but elif/else if aren't supported as of
October 13, 2026
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
