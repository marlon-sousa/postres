/*
    def 007: rust docs
    Lines following //! in the beginning of the crate are recognized by rust as rust docs.
    Lines following /// before something are also recognized as such.
    But what is rust docs?
    This is the way rust offers to document your code, much like doxygen and javadoc do, with their special comments.
    By running cargo doc (see 022), a nice documentation of your code will be generated
    Rust has the philosophy that documentation should be placed as close as possible to the code it documents. This is also used by other languages ... but there is something else very special about rust docs:
    If you include examples in your documentation, they will be compiled and tested when cargo docs is issued, and you should issue this as part of your continuous integration process.
    Why is this useful? Because it breaks the argument that documentation is not useful or desirable since developers will forget to update it anyways.
    If any example in the documentation stops compiling because the implementation changed or their results change because even though the api didn't change its behavior did, you will have a nice error which will remember you to fix the examples and *** hopefully *** review the documentation.
    Documentation following //! in the beginning of a module should give an overview of what the module is used for.
    Documentation before enums, structs, functions or other rust blocks should document, hopefully with examples, how these functions work.
    The best your code is documented the best are the chances  your code is of good quality.
    Several emprovements (and even bugs!) are can be detected when you are trying to explain to soneone else how something works, include by providing working examples.
*/
//! postres
//! Converts postman collections to restclient format
//!

/*
    def 008: modules
    Let's talk about modules in rust, something that might be extremely confusing at first
    A module is in practical terms the logical place where structs, functions, implementations, traits, enums or any other rust construct must reside.
    As simple as that.
    Or better said, almost as simple as that.
    The main file (main.rs and or lib.rs) are considered modules without someone having to declare them as such
    If you want to use other files, you have to declare them as modules inside either the main module or of one of the modules declared in the main module, or they will simply not be considered.
    Below, we declare some modules. If you take a look at the project structure, you will notice that for each module declared here there is a file with the same name and with .rs extension.
    Everything inside a file declared as a module is part of that module.
    The Config struct declared inside config.rs is a struct called Config belonging to the module config.
    Modules can also have sub modules, with content accessible only on the parent module.
    Modules can be declared in some ways. The first one is what we are seeing below, mod xxx;
    In this case, the module content is assumed to be on a file called either:
    * xxx.rs, at the same level of the file where the mod is declared;
    * xxx/mod.rs, with xxx as a folder in the same level as the file declaring the module.
    Submodules of a module (modules declared inside a module) must reside in the following places:
    * If the parent module is defined in a mod.rs file, submodules must reside in the same folder in a file with the name of submodule.rs or in a file called mod.rs within a sub folder with the name of the submodule
    * if the parent module is defined in a file with its name .rs, the submodule must reside in a folder with the name of the parent module and have the submodule name .rs
    The second way is when you declare a inline module. This takes the form of mod xxx { content }
    Inline modules ar exactly as if a submodule with name xxx was defined in a file xxx.rs, but instead of defining the contents in another file they are simply placed inline inside the mod block.
    Now that you know all these boring things about modules, we need to talk about what modules do: they group content.
    This grouping implies in two main consequences:
    1. you can control visibility of a whole module. pub mod is public, mod (without pub) is private, meaning that only its parent can access content inside it.
    2. You have to refer to something defined inside a module prefixing this thing with the module name, so modules provide a namespace functionality.
    3. But if you don't want to type module1::module2::module3::DeepThing, you can use ... well, use!
    Exactly. this is what the keyword use is used for in rust. use module1::module2::module3::DeepThing; will allow you to refer to DeepThing as if it was defined in the current module.
    Amd if you are thinking then that each crate defined in cargo.toml appears as if it were a global defined module, your are absolutely right.
    This is why the line use anyhow::Result; appears on main.rs, even though main.rs does not declare a mod Anyhow. This is provided to us by rust
*/

mod config;
mod converter;
mod error;
mod logging;
mod postman;
mod restclient;

pub use config::Config;

/*
    def 006: lib.rs and main.rs files
    This is the main module of our library, while main.rs is the main module of our binary
    we separate this because other people might be interested in converting from postman to restclient in other use cases.
    Excelent candidates are, for example, integration tests!
    we will expose one main function, the converter itself and another accessor, which will load
    the postman file, convert it and record the converted http file
    because these are public functions and are in a library, we must document them in a way rust doc understands, so that other people interested in the function can quickly see how they work.
*/

pub use error::PostresError;
pub use postman::v2_1_0::PostmanCollection;
pub use restclient::HttpRequests;

/// takes a PostmanCollection and returns a HttpCollection, understandable for RestClient
pub fn postman_to_http(
    postman_collection: PostmanCollection,
) -> Result<HttpRequests, PostresError> {
    Ok(HttpRequests { requests: vec![] })
}

/// loads an input file with a PostmanCollection and records on disk the converted file with a http collection, understandable by RestClient
pub fn postman_file_to_http_file(config: Config) -> Result<(), PostresError> {
    let postman_collection = postman::load_from_path(&config.source_file)?;
    let http_requests = postman_to_http(postman_collection)?;
    Ok(())
}

/*
    def 009: conditional compilation
    We are declaring an inline module (see 008)
    But, before doing that, we have a somewhat curious annotation. What is this #cfg()?
    This is a conditional compilation annotation, similar to our incredible #ifdef in C language
    We will not explain all conditional compilation power here, you can look for documentation.
    We will, however, explain that this annotation affects the thing right below it, and the definition of thing can be better defined as an unit.
    A line, a function, a block, a module ...
    We will also explain that, unlike in C language where you can #define whatever_you_want, rust has some rules on what can be defined.
    You can define your own kind of symbols, which we call features, but there are also symbols defined by the rust eco system.
    One of them is the compilation mode. And when you issue a cargo test, you are defining the "test" symbol, proudly intermediated by cargo.
    This means that "things" defined after a #[cfg(test)] will be compiled only when the "test" symbol is defined, otherwise not.
    As you see, we have whole modules under the conditional compilation test.
    When you issue cargo test, cargo will scan your full codebase looking for functions annotated with #[test]. Because the "test" symbol is defined, conditional compilation will include modules like we have below, where test functions are defined.
    But why specify a whole submodule?
    Because if functions were directly on parent module they would be included in the release executable!
    By creating a specific submodule that compiles only if the "test" symbol is defined, we make sure that our code have the tests when compiled in a compilation with tests enabled, and that the code does not contain anything unneeded in production or debug compilations
*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
