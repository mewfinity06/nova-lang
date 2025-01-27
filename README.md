# Nova-lang

## The Nova Compiler

This is my compiler.
For an unfinished language.
Made by me. Who is not a language developer.
Everything is subject to change, so reader beware!

## Inspiration

C and Rust are my favorite languages.

## Nova Definition Language

The [Nova definition language](https://github.com/mewfinity06/ndl-parser), or NDL, is how you can customize your Nova experience

## Semantics

### Hello, World

```rust
use io::print;

// If a function takes in no parameters,
// why need the parentheses?
func main : {
    print("Hello, world!\n");
}
```

### Anatomy of a Function

Now, this is an important section because SB takes a different approach to functions

1. Every function must have a declared return type; if it returns nothing, mark it as `None`
2. Functions do not need to have declared input
   1. 'Well, what about struct functions?' you may be asking yourself. Fret not! `self` is assumed by default. If you don't want that, include the parentheses.

```rust
func foo : Int {
    return 5;
}

func bar(name: String) : {
    io::print("Hello {}!", name);
}
```

## FAQ

### How real is this project?

Don't expect too much from me! I am very unreliable when it comes to development. I would love for this language to go somewhere, but that might not happen. Everything is very tentative, so be warned.

### Can I add to this project?

Yes, of course! I appreciate every little bit of help. Submit all of pull requests you want, and you will be added to the Contributers section!

### What is the file extension?

#### `.nova` - The Nova file

#### `.nova.header` - The Nova Header file

I like the header file system C and C++ have. I think it allows for a clean declaration of functions, return types, and input types.

I understand that not everyone likes these, so they are completely optional. There will be full support for these files to not be included.
