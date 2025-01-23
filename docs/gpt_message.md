# GPT Message

This is where I will write my GPT messages for reference.
Also it allows me to write them better because I am always accidentally hitting enter
This may also be helpful to see what some of the example files do in `Grammar generation`

## Grammar generation

I am currently writing in my own language, Nova, in Rust. I have my tokenizer finished, which I will provide (it is split into three separate chunks:  mod.rs, which contains the lexer; token.rs, which contains the implementation of the tokens; and token_word.rs, which is a translation unit for converting symbols, like '=>' to a token kind). Now I am onto AST generation and Parsing. I need to write the grammar for the language.

I will provide a few examples to assist in your thinking, all Noav files end in `.nova`:

* `hello.nova` -> A simple Hello, World program
* `employee.nova` -> A simple program to show off Structs and their quirks
* `control_flow.nova` -> Showing off simple control flow functions in Nova
