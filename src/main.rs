use anyhow::Result;
use clap::Parser;
use fancy_regex::{Captures, Regex};
use lazy_static::lazy_static;
use tracing::info;
use url::Url;

mod error;
mod logging;

use postres::Config;

const APP_NAME: &str = "postres";

/*
    def 001: command line parameters handling
    We need to handle command line parameters, that stuff you pass after a hipen to init programs.
    Specifically, we need to know which file is our postman source and on which file we will record the converted http file, which is what RestClient uses.
    In rust, we use the clap crate to help us with this.
    More specifically, we use procedural macros to generate code around a struct which has the representation of arguments we need.
    You can take a look at the derive api documentation on Clap crate. This is just to give you a high level overview of what we are doing here.
    If you are curious to see what a macro expands to, use cargo expand.
*/
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, value_parser)]
    output_file: Option<String>,
    #[clap(short = 'f', long, value_parser, default_value_t = String::from("input.json"))]
    postman_file: String,
}

/*
    def 00: general design

    This is a postman collections to VsCode restclient converter.

    We usually convert something from a format into another format doing the following:

    1- Parse the original source from a file (of the source format, following the source specification) to a neutral, abstract memory representation.
    2- Analyze this memory representation and generate the destination file following the destination specs.
    3- If sometghing is wrong, report errors.

    In fact, this is a simplified explanation of what any programing language is composed of. First, either a compyler or an interpreter reads the sources, which must follow the language specification.
    Then, these sources are parsed (or translated) to a in memory representation of what they are supposed to mean).
    Finally, for compiled languages, a destination file (also following a known specification) is generated. If anything goes wrong, errors are reported.
    Interpreted languages do not write a destination file. The in memory representation is executed, so for these languages the in memory representation is the destination.

    We will do something similar. The module postman is specialized in collecting postman collection files and parsing them to an in memory representation.
    The converter module then is responsible for reading the in memory representation of the postman collection and creating a representation of the equivalent http request used by RestClient.
    The RestClient module is, finally, responsible for converting that representation into a file following the restclient specification.
    If you think about it, we first read a file, convert it to a memory spec, read that memory spec and convert to another memory spec and convert that memory spec back into a file.
    We could simplify things and work with only one memory spec. File / memory spec / file.
    Why didn't we do that?
    Because of coupling. The parsed postman collection might change (a new version of postman specification, for example). The same way, the possibilities offered by the RestClient extension could also change (new ways of defining variables, for example).
    This is by no means a preciosism: RestClient is under active development, and so is postman. If we used only the postman in memory specification, changes in it would likely break the parts which would write the RestClient file.
    If, in the other hand, we wanted to add something new from the RestClient side, we would change code in the same module which parsed the postman collection.
    We, therefore, have choosen to build a middle module, the converter, which knows how to convert from one  memory model to the other. These memory models doesn't even need to know one each order, so postman and restclient are by no means coupled, they can take care of their own businesses.
    The converter, in the other hand, has to know the both sides and make the translations.

    These modules are defined in lib.rs. See 008 to have a better explanation on modules.
*/
fn main() -> Result<()> {
    let args = Args::parse();
    /*
        def 002: printing
        Let's talk a little about printing:
        Printing is cool, is an easy way of debugging and communicating.
        But printing is also synchronous. What does it mean?
        It means that every time you call println!() or stuff alike, in the vast majority of situations, your thread has to stop, ackire a lock (which will happen only when no other thread owns it), print and them release the lock.
        While other threads have the lock, your thread is waiting, doing nothing.
        While your thread has the lock, other threads needing the lock are waiting, doing nothing.
        If you think this has a potential to slow down hardly a software which has threads which print you are right.
        Now, consider that the way must software running on containers use to log is ... printing to stdout, and that logging is a must to any production tailored software.
        Right, we have a problem. But the problem lies in these strange people who made printing synchronous. Why does it have to be synchronous? Let's make it asynchronous, or at least let's make it not rely on any locks at all!
        Well, if we made prints not rely on locks, every threads printing would place characters in the buffer in the order they are produced.
        If a thread was printing hello world, it could by paused by the operating system right after printing l, and the next thread the operating system resumed could start printing "what a beautful world"
        After our thread is resumed, it would print "lo world". The reader (the poor human trying to analyse logs) would then read a ,mangled thing, like "helwhat a wounderful worldlo world". Fun, but not effective.
        Right, we have to depend on locks. Now what? Are we going to wast that many processor cores we have only because we need logging? Unacceptable! Let's not log at all!
        Ooops, calm down. We still need logs, and there are ways around it.
        Suppose we want to print lots of logs. Why don't we put them in a buffer and them let a specific thread assigned to print that buffer to stdout?
        Threads pushing to the buffer wouldn't need to wait for any locks. In the other hand, the thread responsible for printing would need to wait on locks, but it would not slow down the threads that need to print stuff. These will just place messages in the buffer and follow their normal operation.
        It turns out that this is exactly what the logging subsystem does here.
        You will need to read documentation to better understand how this all works, but at least you have the high level concept of what is going on.
    */
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let subscriber = logging::get_subscriber(APP_NAME, "info", non_blocking_writer);
    logging::init_subscriber(subscriber)?;

    info!("program started");
    postres::postman_file_to_http_file(args.into())?;

    Ok(())
}

// following trait converts from source to dest
// once implemented on source, you can call source.into() everywhere a dest is required. For a greater discussion about traits, see 014
impl From<Args> for Config {
    fn from(source: Args) -> Self {
        /*
            def 003: blocks returning values
            this block will assign the result of the matching to the dest_file variable
            To have more information about pattern matching, see 016
            if output file is Some, will will use the value wrapped in the Some variant.
            if it is None, see comments inside the block
            The important thing to notice here is that in rust, blocks such as if / else and match return values.
        */
        let dest_file = match source.output_file {
            Some(o) => o,
            None => {
                /*
                    def 004: regex
                    if output is not provided (it is None), we will replace the file extension from the input file from json to http and use this as an reasonable default
                    We however have a problem: if the source file contains one or more .json part on its content, we want only the last one to be replaced by .http
                    this is not an easy task in rust: manipulating strings is not something easy at all, so we will use regex.
                    It so happens that replacing only the last occurrence of a matched text is also not something easily done in rust
                    in order to achieve that, we will need to use a regex feature called look ahead.
                    The idea is simple: we want to capture some text only if we know that no more occurrences of that same text can be matched to the right of the original text
                    This way, only the last .json is matched.
                    It so happens that Regex, the di-facto rust Standard crate for regex does not support look ahead.
                    Because of that, we will use another well maintained crate called, well, fancy_regex.
                    A regex are created with Regex::new.
                    The parameter to new is the string representing the regex (not the string the regex is applied in), and, if this string represents a valid regex, it is returned as an instance of the Regex struct.
                    We call this process of evaluating the string representing the regex and returning an instance, if the regex is valid, regex compyling.
                */
                lazy_static! {
                    /*
                        def 005: singletons
                        Why static_ref?
                        Because compyling a regex is not something neither fast nor simple.
                        But if you think about it, we just need to compile regexes the first time we use them.
                        If they contain an error, we panic. If they are correct, we don't need to compile them again and again everytime we want to use them.
                        Static stuff in rust is not managed in a simple way. Specially complex static objects which might require heap allocations.
                        The lazy_static declarative macro, in the crate with the same name, provides us with this exact functionality: the expression after the equals sign will be run only the first time this function is called.
                        From the second time on, the RE static will "remember" its value
                        This is, therefore, a kind of singleton.
                        Rust offers other possibilities, and the way you use singletons might be either totally safe or put your code into a trap it will not recover from easily.
                        In this case, we are creating a singleton which will compile a regex the first time it is used and provide the compiled version for use the next times it is called.
                        After the first use (when the stat of regex will go from not compiled to compiled), this is never going to change.
                        This is also not global, the RE variable is only available inside this function.
                        So, in our opinion, this is both safe and desirable.
                        Under the hood,  this will envolve a synchromization feature, in order to make sure that if this function is being called by multiple threads only the first call will compile the regex, while all other threads will have to wait and receive the already compiled regex for read only purposes when its compilation ends.
                        But once it is compiled, because all threads will only read (not change) the data, access is almost instantaneous, way faster than having to compile the regex everytime
                    */
                    static ref RE: Regex = Regex::new(r#"(\.json)(?!\.json)"#).unwrap();
                }
                RE.replace(&source.postman_file, ".http").to_string()
            }
        };

        Self {
            dest_file,
            source_file: source.postman_file,
        }
    }
}
