# Demo lang
Simple language with minimal syntax but high level enough to do cool stuff.


This is a weekend project just for fun.

## Code examples

Hello world:
```
print "Hello world"
```

Variables:
```
// Declare a variable
my_var = 42

// Override variable
my_var = 5

// Use var
print my_var
```

Functions:
```
// functions are defined using lambdas
my_function = { my_int, my_float, my_string, my_boolean |
    print my_int
    print my_float
    print my_string
    print my_boolean
}

my_function 1, 2.34, "string", True

// Arguments are dynamically typed and functions have a fixed number of arguments, the following code is invalid:
my_function 1, 2.34, "string"
```

Data types:
```
// Literals
1 /* Int */
1.23 /* Float */
"1.23" /* String */

// List
[1,2,3]

// Tuples
(1, "hello", [1, 2])

// Enum
type Boolean = True | False
println True

// Record
type User = User(name, email)
User "juanito", "j@mail.com"

// ADT (custom type)
type List = Cons(value, next) | Null
```

Builtins:
```
// function taking a boolean and a lambda
if condition, {
    // code
}

// function taking 2 lambdas
while { /* condition */ },  {
    // code
}

// run lambda 10 times
repeat 10, { i |
    // code
}

// iterate list
foreach list, { index, value |
    // code
}

// iterate list
each list, { value |
    // code
}
```

#### Note:
Currently semicolons are optional but cause some weird edge cases.
Commas separating arguments in a function call are optional when the arguments are clearly delimited:
```
if condition, {
    // code
}
if condition {
    // code
}

each list, { value |
    // code
}
each list { value |
    // code
}

repeat 10, { i |
    // code
}
repeat 10 { i |
    // code
}
```  
