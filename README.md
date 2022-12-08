# postres
postman collections to VSCode RestClient converter

## Introduction

Hello, welcome to postres. Thinking that this is a strange, personal way of writing a readme document?

Well, get used to it. Not only this is true, but the whole project, including source code and personal documentation, will have this conversational style.

To understand why, you need to know that this project is written in Rust, a programming language which has become my preferred one.

This project is, then, not only a postman to restclient converter, it is also my way of teaching concepts of Rust in a way I haven't found elsewhere and which would have helped me a lot if I had found it.

This is also my way of thanking Rust authors and community. Not everyone has the same background, the same habilities and the same perspectives on things. By trying my personal approach to introducing Rust, I hope to contribute to the growing Rust eco system.

This project is not a super low level thing. Neither have your projects to be if you want to write them in Rust.

In fact, in my opinion, one of the most conter-productive aspects on Rust is the notion, many times accepted inplicitly for both Rust community it self and new comers, that Rust has to be low level, has to be hard, has to be for extremely experienced programmers bo be worthy.

Rest assured, it hasn't. It only has to be supper low level, super hard if what you are writting is super low level, super hard.

What changes in rust is that even if you're writing higher level stuf, such as web services, command line utilities or serverless functions, you can have a fluent experience with umpprecedented performance and easiness of deploiment, all of this without even having to touch the more low level aspects of the language.
You will also have many guarantees that your code is safe (no can not read properties of undefined or null pointers exception), all for free while in the other hand because of the typing system your intelicense will work nicely.

Write like typescript, deploy like golang and enjoy the great eco system of crates (libraries) which will solve most part of demands you have, while allowing you to concentrate on wwhat you have to do.

### How it works

This is under construction. Writting explanations is slower than writting code, although I really believe your code gets better once you at least think in writting it like if it were a tutorial.

After all, I don't write code to show the world "how good I am", I write code to be easily understandable by the most number possible of other programmers, so that everyone can contribute safely and condfidently, making cmpanies I work for more productive and pleasant to work in.

There are cases in which we need to write something hard or in a hard way. Take optmizations for example, which might make an implementation to follow a very unusual, non obvious path. This might really be necessary, but it has a cost: code is less understandable, harder to maintain and might be more error prone. If costs are worth, we optimize, and write a nice text near the non obvious code trying to explain what is going on. But not everything has to be optimized, the same way not all things have to go through a complex architecture, only because it was found in some book written for someone somewhere.

But when even your not so optmized code performs better than a super optimized code in other languages or it performs predictably without you having to write a micro kernel or some plugins in interpreters to achieve that, then you start paying attention to a language. This is rust.
Even not optimized rust (use of clones everywhere for example) is likely to perform better or more predictably or both than code written in other languages, meaning that your code style can be kept and you will have significant wins, all proudly provided by rust. This is already a win and it's not a problem unless you want to write an operating system, and this is not the case here, nor it has to be for you to start writting rust.
All of that without, agaim, touching code safety, which is something intrincic to rust im a way I at least have not seen anything close to in other languages.

Being more humble is something I have been strongly trying to achieve. It is my personal opinion that this is a characteristic which makes everything, including code, nicer and more robust.
By using a language which sets clearly the rules of the game, I might miss some freedom, but in the other hand I know my code is safer and less likely to have bugs which can harm someone else's (including my self) lives.
Being a c++ programmer for many years, I have learnt that making mistakes when programming is easy, and that these can really hurt systems and users. Rust helps me to not have that mistakes in an elegant way. Dangling pointers and most part of threading related problems are strongly mitigated by Rust design. As a programmer, I have to be thankful. I don't need to be my self the best of the world to make a program which likely works if all of us can use a language with reasonable approach to comon problems we all fall into some tines. If you're a genius and don't need nothing like this I wish you all the best. If you are only someone normal who is just trying to write cool programs and be a good profissional, then come with us in this journey. It is my hope you will not reggret!

All this to tell you that it is really possible to write Rust in a way that ressembles very high level programming languages ... and getting a nice compiled binary which is likely to perform almost as if it were written uin an exotic, incompreensible way which only a few genius could ever understand.

We will discuss now the general architecture of the project. Implementation details are found in comment blocks in the code, always prefixed by "def xxx: yyy", where xxx is a number and yyy is the description of the subject.

Comment blocks like these explain a concept, discuss why this concept is needed and contextualize the reader of how the below implementation uses the concept. Implementation is found soom after the block.

It is my hope that by presenting a simple enough (but not so simple) project, written in a very fluent way (no complex architecture) that a reader can read from the beginning to the end and face together with the author real problems to be solved, I can show some features of Rust and explain how they work in a unique way.

If you have never programmed before, this can be a little challenging to follow. If you ahve programmed in any language, you should be good to go.

I advice everyone to look for concepts explained in the code in rust documentation (specially the book), which offers a more complete overview. But, the same time it is more complete, it is also more generic, in the sense that the concepts are not (and this is expected) placed in a real problem solving project.

After all, I really hope you enjoy.

Go to src/main.rs, the entry point of all rust executables, and look for the 00: design overview block, where we will discuss what we are doing and how we will do it.

### what topics are explained

- [x] 000: project design
- [x] 001: command line parameters handling
- [x] 002: printing
- [x]  003: blocks returning values
- [x] 004: regex
- [x] 005: singletons
- [x] 006: lib.rs and main.rs files
- [x] 007: rust docs
- [x] 008: modules
- [x] 009: Conditional compilation
- [x] 010: iterators
- [x] 011: builder structs with derive_builder
- [x] 012: post build functions
- [x] 013: enums
- [x] 014: traits and how rust approaches common problems solved by inheritance and polimorphism in object oriented languages
- [x] 015: The Option type (no more nulls in safe code)
- [x] 016: patterm matching
- [x] 017: Generic programming
- [x] 018: destructuring
- [x] 019: procedural macros
- [x] 020: pointers, references and lifetimes
- [x] 021: cargo.toml and cargo.lock
- [x] 022: inference and the _ symbol
- [x] 023: early return with the ? operator
- [x] 024: is rust really easy or you're sheating me? (a quick reflection on programming)
- [x] 025: trees
- [x] 026: An abstraction example
- [x] 027: the stack and the heap
- [x] 028: Memory leaks and dangling pointers
- [x] 029 - pointers, part 2
- [x] 030:     Copying and moving
- [x] 031: pub versus pub(crate)
- [x] 032: weak references
- [x] 033: Data initialization
- [x] 034: inner scopes and shadowing
- [x] 035: dereferencing and the Deref trait
- [ ] A little bit of devops: github actions to generate releases

## Getting started

To take a closer look at this project, you will need rust. Only that. No docker, no multiple packages and hard configurations, only the rust eco systems.
If you're using Mac or Linux, you will need to run a shell script and it is done.
If you're using Windows, you will need to install some developer base libraries, because these are not provided by default on Windows installations.
Regardless of your operating system, access [rustup.rs](https://rustup.rs/) and follow the instructions. The process is kit authomated and easy, way easier than what other low level languages usually require.
Once rust is installed, clone this project. Then change dir to the folder where it has been cloned and perform a cargo check to make sure your rust setup is working.

We strongly suggest the use of [VS Code](https://code.visualstudio.com) as your base editor.
If you're going this route, install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Contributing

Feel welcome to make suggestions and contribute fixes. Remember that, more than a conversor, this project is really aimed to help programmers to join rust, demistifying the language at the maximum we can.
The more rust is used for daily jobs the better software is likely to be in general. Suggestions and fixes are therefore accepted in all aspects, ranging from implementation to explanations and other aspects.
Also, please keep in mind that this is being built on spare time. In general, if you are:

1. A beginner in rust.
2. A pationate for the language wanting to help to make it more accessible for new comers.
3. Someone interested in programming teaching, specially in exploring less traditional ways of explaining concepts.

you will feel at home.

## Remarks

I am not writting a Rust book here. I am, definitely, still not in a position where I know absolutely all things about rust to write such a book, and there are several very well written books.

But, the same way as I don't know every single detail of rust and even so I wrote some very interesting code for my company, you can also write code for your company and benefit from everything rust has to offer without being a genius programmer.

For experienced rustaceans reaching this project, your super qualified advices would be more worth to the community than your super qualified criticisms. But, even if you want to criticize, create issues and I will make my best to comply.

I also warn everyone that I will not complicate things when I don't have to, because this is my personal approach on software writting.

As a last advice, I will not be explaining very common concepts a programmer can understand by context (not explaining what if blocks are, nor giving a list of operators). However, I will try to explain everything which a programmer not fammiliar with rust would look at and find strange (such as match blocks for example), at least the things we will need in this project.

## Special thanks

Initially, I based the parsing of postman collections on the [postman-collections-rs crate](https://github.com/mandrean/postman-collection-rs). I thamk the author for the great work they have done.
However, due to several changes I needed, specially to comply with some parts of the spec which were not covered, I decided to base some parts of the parser on their code, thus the credit for them and their license also applies to parts of this project
