# LiaX

[Pronounced `/lɪə ĕks/`] A small Lisp language that serves as a preparation for implementing the biggest project of my life.

## What is the point?

LiaX is a small Lisp language for me to prepare for the biggest project of my life, a new functional programming language. I don't know how this will turn out, but it for sure is going to be my biggest project (a few thousand lines of code?) until I actually start the main language project.

## What's the point in another Lisp?

If this turns out to be a good end product, I'm going to use it in my daily work for scripting small tools and tasks for a while (until I get the main language going).

However, please note that the main purpose of this language is educational. I need to implement this in order to learn about several domains I'm going to need to implement the big project.

## Which domains is this going to help me learn?

- Interpreter (REPL + interpreted step-by-step execution of code from files) development;
- Advanced parsing in Rust;
- Haskell and advanced parsing in Haskell;
- What generally goes into creating a scripting programming language;
- What it takes to implement the most basic standard library of a scripting language;
- How functional languages are implemented and what I personally want from my personal "best" functional language.

## Explicit goals

- Have a basic Lisp language ready;
- Both REPL and interpreting scripts from files;
- Have two separate identical implementations of the language: one in Rust, one in Haskell (probably in two separate branches);
- Implement standard library sufficient for implementing basic scripting tasks like file IO, printing, functions, basic data types (lists and hash maps?), basic functional algorithms as functions (filter, map, reduce, fold, etc.), work with strings, maybe basic math.

## Explicit non-goals

- Implementing all those Explicit goals in the Lisp language itself (any degree of self-hosting) is not planned at this time; most if not all functionality is planned to be implemented in the language of implementation (Rust and Haskell);
- Creating a fully stable and mature Lisp with its own ecosystem is not the point, the point is to learn how to implement a bigger functional language via starting small;
- This project has explicit end goals: Implement a Lisp with a basic standard library in Rust and Haskell (separately). Therefore extending and maintaining it for a long time is not a goal. It's possible that if I find the language useful in the future, I'm going to return to it at some point, but for now it's not planned.

## Tracking progress

As the project has explicitly defined boundaries and is not planned as a years-long endeavor, it has an end date (hopefully a relatively near one), and as such shouldn't need involved issue tracking via GitHub issues or any project management system. Therefore, the progress is going to be tracked via marked comments (the standard TODO, FIXME, BUG, etc.). If you wish to see what state the project is in, just `grep` for `TODO`s and `FIXME`s in the code and you'll probably get the picture from those.

## Licensing

The licensing is MIT, which is usual for open-source projects. Provided you redistribute the MIT license and the original copyright with your code, you can use this however you like.