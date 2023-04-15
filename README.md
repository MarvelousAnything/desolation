# Desolation

## Inspiration
This language was inspired by a language called [Tranquility](https://www.cs.drexel.edu/~bls96/tranquility) that was written by Brian L. Stuart.
The structure of the compiler was based off of the structure of the rust compiler.

## Structure
In this section, I will describe the structure of the compiler.

### Crates
  - [desolationc](compiler/desolationc)
    This crate is essentially the entry point for the desolation compiler.
  - [desolationc\_tokens](compiler/desolationc_tokens)
    This crate contains the definitions of the tokens used by the lexer and ast.
    I decided to make the tokens common; I do not know if this will come to bite me in the butt. 
  - [desolationc\_lexer](compiler/desolationc_lexer)
    This crate contains the logic for reading a source file and producing a tokenstream.
  - [desolationc\_ast](compiler/desolationc_ast)
    This crate contains the abstract syntax tree.
    This tree is built from the tokens in the tokens crate and constructed from the token stream in the lexer crate.
    **This crate does not actually produce the ast. That is the purpose of the parser.**
  - [desolationc\_parser](compiler/desolationc_parser)
    This crate converts a token stream into an abstract syntax tree.
