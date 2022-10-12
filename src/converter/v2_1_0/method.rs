use std::str::FromStr;

use strum::EnumString;

use crate::{error::PostresError, postman::v2_1_0, restclient};

pub(crate) fn convert_method(
    postman_req: &v2_1_0::RequestClass,
) -> Result<restclient::Method, PostresError> {
    let method = postman_req
        .method
        .as_ref()
        .ok_or(PostresError::PostmanRequestMethodNotPresent)?;
    let postman_method = PostmanMethod::from_str(&method)
        .map_err(|e| PostresError::invalid_postman_method(&method))?;

    /*
        def 014: traits and how rust approaches common problems solved by inheritance and polimorphism in object oriented languages

        In the next line, we call .try_into in the instance of the PostmanMethod enum.
        But hwhere it is defined?
        In nowhere related to the enum itself. It is, instead, a trait implementation.
        To better understand this, look for the next block marked with def 014 to understand how traits work.
    */
    Ok(postman_method.try_into()?)
}

/*
    def 013: enums
    The author of the postman_collection crate, the original crate we based our postman collection parsing on, specified request methods as an Option<String>
    While this is technically correct (in the postman collection json this is a string), postman specification specifies that this string has to have the value  of one of the specified methods
    Enums are the feature we use to specify that a given value has to be one of the values pre defined.
    In rust, enums go a little beyond that, because they can specify that a value can be of one of the pre defined types, in something close to what other languages call unions.
    In this case, however, we are using an enum to specify that a string (the method field), can have only the list of accepted methods.
    This enum does not encapsulate any specific type, it is composed of a list of possible tags that have no other meaningful associated value.
    In order to assign the correct variant, we would have to:
    1. Get the string with the method coming from postman collection.
    2. Compare it with the literal "COPY". If it matches, we return the variant PostmanMethod::COPY.
    3. If it doesn't, we try matching it with the literal "DELETE". If it matches, we return the variant PostmanMethod::DELETE.
    4. We keep trying all possible other variants. If the string doesn't match any of them, we return an error, because clearly whatever was in that string is not a valid method, or at least not a method we know how to handle at the time of programming.

    If you think this is a lot of work you're correct. Validations often require lots of work.
    But in this case, fortunately, the strum crate offers us, by means of a procedural macro, this code auto generated.
    As a final thought, I know I could just add this enum to the collection parsing, but I wanted to talk to you about enums and this was a good place to do so.
*/

/*
    def 019: procedural macros

    Ok, another long block, I tell you from the beginning.
    But you want to take a look at rust, right?
    So stick with me and let's understand procedural macros and why they are a game changer in rust

    Some times, we want to add functionality to some code we don't know.

    Think about it for a moment: you have, for example, a http handler. You want to attach this handler to a route.
    You, as a framework maintainer, could require your users to add the handler address to a list of known hhandlers, may be a hash map for performance with an index containing a path.
    Then, in another place in the code, you might require all paths (which can be regexes) to be defined and pre compiled on service startup again for speed.
    All well and good, except that this is repeated code that your users will have to repeate for all handlers, all the time.
    Well, you could also require that your users declare every configuration aspect, such as path definitions and path to handlers associations on external config files, perhaps xml ... and now your users have to know not only your public api but also an exotic xml schema that you have defined from within your head.
    Does this remember you of some managed open source languages still used and how bad it was to keep xmls and code scatered in a basilion of different files, classes, packages, only to tell to a specific standard that the /ping path should call your handlePingController?
    I don't miss neither that times nor that languages,     and you also don't need to.
    Good, there is another way: if we have our code interpreted, we then can attach lots of metadata on it and stablish a pattern: if we got a /ping path, we will look dinamically for a handlePing function and call it, problem solved!
    Except that performing that lookup all the time is slow, and if the search code is too generic some outsider can try to call random functions at your code.
    Ok ... then what? There is no way, openning the xml editor ...
    Hold, hold! What we need here is to, some how, mark places of our code with some special meaningful symbols and, when compiling, we generate some extra code alongside it. This code generated alongside it is also compiled.
    So that the final compilation has your handler and also entries on every where else they need to be to register it as you whish.
    If it is still generic, think about it as you hiring your nasty teenager cousin and asking them to write down all the registration code exactly the way it should be for each of your handlers to configure them as you wish, for all your handlers all the time.
    But, some how, think that this teenager never ever gets it wrong and never complains of doing that boring work ... to much for you?
    Well, procedural macros are exactly that.

    This is important, because the hard part is understanding the concept here, as it is kind of unique to rust the way it is implemented.
    Let's talk about another situation: JSON deserializing.

    You have a nice JSON file, this is a text file, nothing more. You have a struct in memory that represents for your program that same information.

    You have to load that text (or string) into your struct, so that you can use it (read, write, as usual). But how do you do it?

    Well, you could again use a dynamic aproach where every time you need a information you go walking characters until you find the key, then you find the associated value, reporting errors during this process. Slow, tedious, error prone.
    You can build a set of nested hashmaps containing keys and values that can be a string, a number, a vector or a nexted hashmap and query at runtime the information you need. Again, querying this dynamically at runtime is slow, and several gets and checking if the results are None or Some all the time is slow (for write, for read and for execut at run time) and tedious.
    Well, you can write a piece of code which inspects your struct at runtime, discovers its fields, parses the json, copies the pieces of the json whose keys matches with the keys of your struct in to the structs values and then use the struct. Hmm, this would make the usage nice, no more gets, checks if results are null, nothing like that.
    The load would report errors from the beginning, preventing you from using inconsistent data, it seems nice. Except that check at runtime for struct fields is slow, requires lots of metadata compiled (no optimizations), and your memory usage is high, because pieces of the original string would need to be copied over to the struct.
    And this is rust: we like speed and efficient memory usage.
    Ok, then, giving up ... impossible, the only other way would be writing for each possible struct a customized code which did the parsing and referenced the data.
    Each struct will need to have its deserialization code written by hand, because each program has its own structs, and uses its own json contracts, impossible for us to know all possible structs and all possible jsons which would be written and used in all the programs written in the past, now and in the future.
    This is why we have to resemble to runtime inspection, right? Because at runtime we already know our struct shapes and can load JSON's on them. Well, rust and JSON don't fit together, and it makes sense, because operating systems don't require JSON reading on their kernels. I am not writting an operating system and need JSON, so I will abandom rust.
    Again, hold on. Remember the procedural macro concept: you hire a nasty teenager to write down all boring, tedious code you need to write ... and this teenager does not ever complain ...
    and it seems though that writting deserialization code specialized for each struct is tedious, boring ... and the only way to load jsons to structs without having to look at fields at run-time!
    Also, because we know upfront that our json string is not going to change and that our struct is known (it is known in compile time, right?) ... then we don't even have to make copies in memory, we just have to reference the JSON itself for the field values.
    Better of all the worlds ... definitely! Welcome to procedural macros!

    Ah, you are thinking, but again, there is no way to write this, because an engine can't know up front what struct the programmer is writting.

    The only way to do it would be, perhaps, if the compiler knew, when compiling the struct, how to generate the deserialization code. But then the compiler would have to, may be:
    1- compile a struct.
    2- realize, some how, that this struct specifically needs to have code generated for json deserializing (we don't want to generate deserializing code for all structs of our program, right?)
    3- Then, stop the compiling process and generate some valid rust code (the deserialization code).
    4- Then, compile the just generated code as a, say, method implemented for that struct or something like that.
    5- Finally, let callers call the method to deserialize the struct which was it self generated by the compiler.

    Seems good. We just have to submit pull requests asking the compiler team to stuff into the compiler code all kinds of on demand extra code generators like the json deserializer. I guess these guys will be happy to accept all of them, and the compiler code will be a breese to maintain, right?

    Hmm, this wouldn't work either, it wouldn't be sustainable. But we are almost there. It is if ... well, if we could instruct the compiler to call a kind of plugin every time it finds some markers in code and then collect the code these plugins generated, join it with the original source code and only them compile everything ... it would be great.

    And ... no, it wouldn't be great. It is currently great!

    Because procedure macros are, literally, small programs someone write that the compiler calls when special markers are found on a source code. These programs receive a piece of parsed rust code and returns back another piece of parced rust code that the compiler then compiles as if it had been written in the original source code. Literally the teenager hired to write nasty, tedious code.

    Again stressing that: procedural macros are programs which receive some parsed code (for example a struct), and generate back some parsed code (for example that struct alongside a impl with a function that deserializes json for that struct) that the compiler then compiles as if that code were written by the programmer.

    And this is how we end up having deserializer code being automatically generated for any struct during compile time instead of run-time. Rust is very good at performing things other languages can only perform in run-time at compile time.

    This is also how we might generate additional code to register a handler with a given path, because there is more to that:

    The markers defining a procedural macro can be parametrized, and the parameters are also passed to the plugin which generates back code. This is how we are handling the procedural macro below.

    Now that you hopefully understand the concept of procedural macros, they can come in thre flavours:
    #[derive(macro_name)], #[macro_name] and macro_name!()
    We won't extend this further more, but the idea is that all of these flavors will call a program defined by macro_name and pass to this program the parsed rust code of the block following the marker in the original source code.
    In this context a block can be a struct, an enum, a function or several other things, deppending on the used flavor.
    There is also the possibility of recursions, meaning that the passed block can it self have another procedural macro call. The compiler will call all the procedure macros respecting the recursion (until a certain limit) and assemble back the returned parsed code, which will be then compiled.

    Now, let's talk about our problem.

    Remember from 014 that we want to build an enum from an external string.

    The EnumString procedural macro does exactly that. It creates an impl of a method called from_str which takes a &str and compares it against all variants of the enum it receives as the original source code (the block passed to it) and returns the appropriate variant or an error.

    This allows us to convert &str to enums with a different implementation for each enum as if we had written this by hand, at build time.
    We call the PostmanMethod::from_str in our code, but if you look for this method definition here you won't find it. This method is generated by the enumString macro specifically for the enum it receives as the original source code and returned alongside with the enum it self.

    Again, there is much more on the documentation. Because you receive parsed code (thus an abstract parsed tree), understanding how these macros are implemented may not be that easy, but using them definitely is.
*/

#[derive(Debug, EnumString)]
// specify that the macro should expect values on the original string in uppercase that matches our enum variants which have, by rust conventions, in camel case with the first letter captalized
// if we didn't do it, the matching would fail, because by default matching is performed against the variants as they are specified
#[strum(serialize_all = "UPPERCASE")]
enum PostmanMethod {
    Copy,
    Delete,
    Get,
    Head,
    Link,
    Lock,
    Options,
    Patch,
    Post,
    Propfind,
    Purge,
    Put,
    Unlink,
    Unlock,
    View,
}

/*
    def 014: traits and how rust approaches common problems solved by inheritance and polimorphism in object oriented languages

    This is a topic which works closely with generic programming, see 017

    What are traits?

    Traits are groups of common functionalities (behaviors) which can be attached into one or more enums and structs.
    As simple as that.

    Before talking about traits, however, we should make a quick recap on object oriented programming, just enough for you tu understand how traits are thought.

    In object oriented programming, you define data structures (specification) representing objects and create instances (implementations) of these objects.

    We can specify that a person, for example, contains legs, arms, body, and head.
    The group of these attributes or properties composing a person is defined in a data structure and treatened as a person.
    When you pass a instance of type Person struct to a function, you are sending the group of properties which compose a person all together to the function, so that the function can act on one or more of the attributes.
    But this is not the whole story: you can also define behavior affecting directly the state of each person.
    This behavior is known as the methods of the object.
    A walk method, for example, will move the legs attributes of the person instance it is called on so that the person's position also moves, just like a human walks.
    A Car has an engine and some pedals. The turnOn method will put the state of the engine of the car instance it is called on into the working mode.

    Methods are special functions which act on instances of objects. In the oriented object jargon, the specification of an object is called class.

    The implementation of classes are called objects, or also instances.

    The account class defines properties and methods of the accounts: userName, password, permissions. I can, from that definition, create several accounts: mine account and yours account are different, our names are different, may be I am an admin and you are not.
    But, even so, mine account and your account are instances of what we have defined as an Account class: both have an userName, a password and a list of permissions.

    The method login belongs to the Account class, this function must take as parameter an instance (mine account or your account or anyone else's account) and act on the properties of the instance it is called on.
    The login method receiving my account will read my login and my password and compare them against a database. The same method called on yours account will read your user name and your password and compare them against the database.

    In rust, we don't have the class keyword. Instead, we have the struct keyword which allows us to define the properties (or fields) composing whatever that struct represents.
    We also have the impl keyword, which allows us to implement methods on the struct.

    As we already said, methods must receive an instance of the struct they belong to, so that they can act on that specific instance.
    In many programming languages the instance the method is acting on is "magically" available inside the method, usually using the reserved "this"keyword.
    Rust, like python, prefers to be explicit, requiring that a parameter called self appears as the first parameter of each method, so that we know we are in a method manipulating the state of the given instance.
    ```  rust
    // account specification
    struct Account {
        user_name: String,
        password: String,
        permission: Vec<String>,
    }

    // implementation of methods acting on Account
    impl Account {
        // will login the account if user_name is a and password is b
        fn login(&self) -> bool {
            // note the reference &self, which points to the account instance this method is called on
            if self.user_name == "a" && self.password == "b" {
                return true;
            }
            false
        }
    }

    fn main() {
        // create an account object or instance from the Account struct or specification
        let account_1 = Account {
            user_name: "a".to_string(),
            password: "b".to_string(),
            permissions: vec![],
        };
        // create another different account object (implementation) from the Account struct (specification)
        let account_2 = Account {
            user_name: "c".to_string(),
            password: "f".to_string(),
            permissions: vec![],
        };
        // now, when we call account_1.login(), the &self the method receives points to the data in memory from the account_1 instance, user_name "a" and password "b"
        // notice that, although we didn't pass parameters in the call, the method still receives one parameter, which is "&self".
        // Rust builds this parameter based on the name of the variable we place before the dot, in this case account_1
        // Because of this, the method compares the user_name field with "a" and the password field with "b", and returns true because both comparisons hold true
        assert_eq(account_1.login(), true);
        // now, when we call account_2.login(), the &self the method receives points to the data in memory from the account_2 instance, user_name "c" and password "f"
        // notice that, although we didn't pass parameters in the call, the method still receives one parameter, which is "&self".
        // Rust builds this parameter based in the name of the variable we place before the dot, in this case account_2
        // Because of this, the method compares the user_name field with "c" and the password field with "f", and returns false because none of the comparisons hold true
        assert_eq!(account_2.login(), false);
    }
    ```

    Good, at this point if you are fammmiliar with object oriented programming you are bored with me. But be honest, did you ever stop to think how the this parameter magically appears inside methods or how it is feed?
    Believe me, I wouldn't be spending neither yours nor my time if understanding how this works conceptually was not important.
    Now, there are other interesting features that object oriented languages provide, which we will need to talk about.

    The first one is inheritance. The concept, again, is simple, and based in genetics.
    The same way you inherited genetic characteristics of your parents, specifications (structs or classes) can also inherite characteristics of "parent" specifications.

    The model is not perfect, as our inheritance is built from unique characteristics of our parents, letting us with some unique combination of both. In the object oriented paradigm, inheritance is based on the concept that if a specification B inherites from specification A, then specification B has everything specification A also has, plus some new stuff (inheritance) or some changed characteristics (polimorphism).
    For me, what helped to make my head around it is understanding inheritance like this: specification B is an adapted version of A, an A which multated and gained some extra characteristics, while maintaing all charateristics of A.
    Way generic?
    Let's take a look at this:

    class Human {
        // some human characteristics
        // we won't define them here, but real methods would
        breath() {
        }

        speak() {

        }

        walk() {

        }

        int numLegs = 2

    }

    // singers are humans.
    // singing envolves breathing and speaking
    // all humans, as per our specification, know how to breath and speak
    // so that singers just have to manipulate their breathing and voice in a given way
    class Singer extends Human {
        // sing method
        sing() {
            // where are the breath and sing methods?
            // the singer specification doesn't provide them, even so they are being called on the singer instance (this keyword)
            // These behavior come from the parent class, Human.
            // Singers are humans which mutated and, using their base behaviors, composed a another one, called sing, that only them have
            this.breath();
            this.speak()
        }
    }

    Inheritance is cool, because it allows our specifications to adapt from more generic ones.
    If I have a button on screen, all the visual click handling is common to all buttons, so that this visual behavior can be defined in the base class, from where our specific button adaptated from.
    What changes in our button version is that our button has adaptated from the base button which didn't do nothing as a click response (it only reacted visually to the click).
    Ours, in the other hand, while behaving visually the same way, define a specific action to the click.

    One more example? If our singer gets sick, they go to the hospital, where doctors know how to handle sickness ... of singers? Nop, of humans.
    Because humans, singers or not, have the same base characteristics, doctors know how to diagnose and treat them independently of their professional backgrounds.

    This is why you can pass a singer instance to methods which are capable of dealing with Human parameters.

    If there are sicknesses that only affect singers, there might be some functions which accept only singers as parameters.

    In this case, trying to pass a Human instance as parameter would throw a compyler error.

    Inheritance can also be indirect:
    I can have a Human, then a VoiceProfessional, then a Singer.
    VoiceProfessionals are adaptations of humans, which have ackired some habilities that not all humans have. These might be radio show runners, actors, book readers ... and singers.
    Singers are VoiceProfessionals who have adaptated to ackuire the sing behavior, which is using their voices in a very specific way.
    A singer will, therefore, has all singers behavior plus all VoiceProfessionals behavior plus all humans behavior.

    Again, I hope you understood well the concept of hinheritance.

    At the end of the day, it allow that functions wich accept a given specification can also accept specifications which extends or inherites from these specification, much like a doctor of Humans can receive a singer as patient.

    But this is rust, and in rust there is no inheritance, right?

    Yep. This is a design choice, because inheritance can itself make things extremely complex.

    The main trouble is in the fact that, sometimes, it is very convenient for us to have a class be treatened in a generic way. We then make it inherite from some other class and are all happy ... except that this relation-ship might not make sense at all to represent our real life model.

    For example, may be we have a Human having only one leg.

    This is a human, but the human specification says that humans have two legs. Further more, the walk behavior will move two legs, but this human specifically have only one.
    What we do now?

    Well, we create one spec, called OneLegHuman, which inherites from Human specification.

    Because we know that the walk behavior from the Human spec does not apply, we define a behavior with the same name, walk, which applies for OneLegHuman specification.

    But now we have yet another problem: suppose we have a function we will name as move. This function accepts Human instances (or anything inheriting from Human spec) and calls the walk behavior.
    Because this function only manipulates humans generically (after all voice professionals and singers are all Humans), it will call the walk method defined in the Human spec.
    As far as it is concerned, it move humans, whatever they might be as [professionals.]
    This wasn't a problem, because right until now we were sure that all human descendants would have two legs, so that the walk behavior defined for Humans would apply.
    But this is no longer true, we have now a special kind of Human with one leg who can walk, but not like most part of Humans do! If we restrict the move function to only accept Human instances, then this function won't be able to move VoiceProfessionals, singers or OneLegHumans.
    This would break one of the main object oriented wins, which is making functions of a base type manipulate inherited types.
    In the other hand, if we use the base behavior any way we will break real life situations handling, such as OneLegHumans still being humans who can walk.

    Polimorphism

    The response for that was polimorphism. It says that every time a descendant specification defines a behavior with the same name as the base specification, the behavior defined on the descendant class is used, even by functions accepting the base specification.

    How that works?

    In a model where functions accepting a base specification only called behaviors on the base specification, we could be sure at compyle time of what method to call, because the only possible option was the method defined on the base specification, even if a descendant specification defined another method with the same name.

    But, with polimorphism, if a function accepts a base spec and a descendent spec is passed, may be the method defined in the base specification must be called, but may be that the method to be called is from the descendant spec, if it has the same name.

    We have no way of knowing all possibilities anymore, unless we, somehow, instruct the function to check the instance it received to try to discover if this is really a base instance or a descendant instance and, if it is a descendant, to check if it has a more specialized version of the method it needs to call.

    Now things are more complicated. They become more complicated if we can make type casts at run-time, and we usually can.

    Polimorphism slows things kit a lot, because what was a method addressing which could be generated right into the compyled code now becomes a kind of table where you have to search for a method name and check if the base or the descendant version has to be called.

    Remember, though, that inheritance can have several degrees (a extends b extends c extends ...) and that if I call a function which works on A instances it has to check several possibilities, because either b or c or ... specs can have a more specialized version of the method.

    For languages worried with performance, this is an expensive price to pay, specially by default. This is why c++ lets you specify if a method is virtual (it should be called on the descendant ratter than on the base class) by functions accepting a base class, so that you have to opt in for this behavior.
    Other languages force polimorphism and you sinply have no option to opt out.

    We won't be talking here about virtual tables and other way in which polimorphism can be implemented for classes, but you have to keep in mind that the OneLegHuman we talked about is something hard to represent using inheritance.

    we, instead, are going to explore yet another problem: VoiceProfessionals inherite from the Human spec. It so happens that humans with one leg can perfectly be voice professionals.
    Then ... what do we do?

    Well, we can change the Human behavior to walk with one or two legs.
    But what if Human is on a library we do not own? What if it is in a library we own with several users and start sudenly accepting that the number of legs a Human have might be variable?
    We could break a Dancer implementation someone made without ever knowing that, we don't know what kinds of inherites our clients are doing.

    Well, we can make Human inherite from WalkerKind, another base class which can manage walking with a variable number of legs. But then we will have to inherite OneLegHuman from WalkerKind, and we are back to the same problem.

    Even if we have the perfect inheritance chain, our class at the 20th level (this is common) will inherite all kinds of behaviors we have no idea what they are from all the upper chain.
    If we define a given behavior, is it defined elsehwere in the chain? If so, what kinds of unexpected behavior can we have as consequence (remember, all functions, including the ones built to deal with intermediate levels of the inheritance chain, will call now this version of the behavior)?

    This is not to say that inheritance is bad. Nothing in programming is neither good nor bad, things have tradeoffs. Inheritance is used successfully in several domains and many of the softwares you use are likely based in object oriented programs which use it.

    Well, let's forget about inheritance based modeling for some instants.

    We have structs (specifications) and impl blocks (which define behavior on that structs).

    How can we add extra behavior, which is, behavior that the author of the struct didn't add their selves?

    Well, we can inherite ... ooops, this is not a possibility, remember?

    Right. Them what we could do?

    We can "teach" the struct some behavior. Then, our functions would receive only "qualified" structs.

    Agaim, the idea is simple: we are a function which moves things by making them walk. What things do we move? We absolutely do not care, as long as whatever is passed to us has the "walker graduation degree", or in other words things that have been thaught how to walk.
    We then call the walk behavior on the thing, and the thing moves. How does it move? We don't care. We delegate the walking to the thing, because we know that this thing is graduated in walking. The only thing we worry about is to instruct the thing to walk where we want it to, so that it moves.

    See, sudenly we are an independent function. We don't work with a Human or a Singer instance, we make no assumptions of what the instance is. We don't assume or expect that humans and singers have any kind os special relation-ships only because they know how to walk, other than the fact that both know to walk. The only thing we care is that the thing, whatever it is, knows how to walk.

    One thing graduated in walking doesn't have any coupling with other thing which also knows how to walk. They are not coupled.

    Singers, VoiceProfessionals and Humans all would have to be graduated in walking, but here comes the point: if my program is dealing only with singers at a given point, I don't even need to model humans in the first place.

    If my program are dealing with Humans and sudenly I need singers, I can teach the humans to sing.

    I can teach Humans, provided by a library, to sing. Humans authors can model anything new without fearing they would break anything, because whatever methods they call on the Human they authored will not be suvbstituted by a newer version of this method I wrote in my project that they didn't even know about.

    In the other hand, methods on Humans will always manage the Humans specification.

    This makes things run faster at run-time because of the method addressing issue. This makes code safer for authors, and this makes the behavior important for us not be coupled with implementations important for others, because each person will implement the pack of behaviors they need for whatever struct they need.

    This also helps to migigate the inheritance by convenience (Humans are singers, birds are singers, so let's make humans and Birds be descendant of a Singer specification). If Humans and Birds have anything in common in terms of inheritance, it will definitely not be related to the fact that both are singers, I hope you agree.

    But what about traits?

    Well, traits are the pack of behaviors you can "teach" some spec!

    Once you "teach" some spec the group of behaviors of a trait, this spec is said to implement that trait.

    ```
    trait Singer {
        fn sing(&self)
    }

    Struct person {
        ... anything here
    }

    struct Bird {
        ... anything here
    }

    impl Singer for Person {
        fn sing(&self) {
            // self here points to a person, so you can use whatever attributes a person has to make them sing
        }
    }

    impl Singer for Bird {
        fn sing(&self) {
            // self here points to a bird, so you can use whatever attributes a bird has to make them sing
        }
    }

        // this function make whatever type sing, given that this type "knows" how to sing.
        // we establish this requirement by creating a type generic we call T and constraining the type acceptance to types implementing the Singer trait (in other words we declare we can receive any type as long as this type knows to sing)
        fn sing_a_song<T: Singer>(singer: t) {
            singer.sing();
        }
        ```

    I hope that the intent of traits is clear. I needed to explain how the alternative, inheritance, works and how traits offered another perspective on the subject.

    Again, traits are not the perfect solution either. Some times you will miss cases in which inheritance would be more elegant.
    Still, given all the advamtages traits offer, I think they are a good choice, and Rust has choosen them, so we need to understand what they are and how to use them.

    Before discussing our needs in terms of traits, I will finalize saying that rust also offers dynamic dispatching as a form of polimorphism.
    Still, by combining traits and generics, the way you've just checked above, we achieve what inheritance does without any run-time penalties unless we need to.

    Let's talk then about the TryInto<T> trait, defined by Rust and used for converting data types.

    We will convert from the postman method enum to the RestClient http_method::Method enum
    This might seem odd or not useful, but let's think about iit:
    PostmanMethod represents one of the http methods postman manages.
    restclient::Method represents one of the http methods RestClient knows how to manage
    Although these methods have the same name among both enums, may be that postman will offer a http method that RestClient still doesn't know how to manage, or may be RestClient will know how to manage a http method that postman does not yet support.
    If we use the same enum for the two cases, we will couple http methods that these two different features support and will get in trouble if we find a case where they do not exactly match
    When matching enums, RUST will make sure, at compile time, that we handle all possible cases. If we latter add another variant to the enum, all places matching objects of this Enum type will stop compyling.
    This will make sure that we don't forget to handle the new variant everywhere we need it!

    The try_into trait is a behavior we can add to some struct or enum to "teach" this struct or enum to convert itself to another format.

    This way, when we need to convert a given instance to another type, we just call the .try_into() behavior and got the converted instance.
    For someone reading the function, a .try_into() call is an elegant way to know that we have a thing being converted to another thing.
    Because we are implementing a trait, the trait implementation (the conversion code itself) is separated from the place in code performing the conversion.
    Because we use a trait which enables anything to be converted to anything else, we can call .try_into() on any object being converted, so that we don't have multiple names for multiple conversion functions, we have something that any rustacean will know instantly that is performing a conversion, letting our code standardized.
    The try_into trait is itself generic, allowing conversions from the type the crate is implemented for to a <T> destination type.
    What type? The type we define in the trait implementation, meaning that I can implement try_into from a type to multiple destination types, providing several implementations to the trait with different destination concret types.
    When we call .try_into() on an instance, the compyler will look for a impl try_into<[concrete_type_of_the_destination_variable]> for [source_type] {} and call the try_into behavior.
    So, to summarize, we will "teach" the PostmanMethod enum to convert it self to HttpMethod of restclient.
*/
// converting PostmanMethod to restclient::Method
impl TryInto<restclient::Method> for PostmanMethod {
    // we are not going to talk about associated types here
    // understand it as telling to the trait that this is the kind of error we will return should the conversion fail
    // this could be also a generic type, much like the destination type is. Instead, the associated type notation is used.
    type Error = PostresError;

    // self (lowercase) is, as we said, the instance this metthod is called on, in this case an instance of type PostmanMethod
    // Self (first letter captalized) refers to the trait it self. In this case, the Result enum returned contains either an Ok variant holding a restclient::Method or an Err variant parametrized with the error type the trait (ence the Self keyword) defines. (type Error = PostresError)
    // The Self (first captalized letter) will always refer to the enum or struct specification the impl block reffers to or to the trait the impl block refers to, if this is a trait impl block
    fn try_into(self) -> Result<restclient::Method, Self::Error> {
        let http_method = match self {
            PostmanMethod::Copy => restclient::Method::Copy,
            PostmanMethod::Delete => restclient::Method::Delete,
            PostmanMethod::Get => restclient::Method::Get,
            PostmanMethod::Head => restclient::Method::Head,
            PostmanMethod::Link => restclient::Method::Link,
            PostmanMethod::Lock => restclient::Method::Lock,
            PostmanMethod::Options => restclient::Method::Options,
            PostmanMethod::Patch => restclient::Method::Patch,
            PostmanMethod::Post => restclient::Method::Post,
            PostmanMethod::Propfind => restclient::Method::Propfind,
            PostmanMethod::Purge => restclient::Method::Purge,
            PostmanMethod::Put => restclient::Method::Put,
            PostmanMethod::Unlink => restclient::Method::Unlink,
            PostmanMethod::Unlock => restclient::Method::Unlock,
            PostmanMethod::View => restclient::Method::View,
        };
        Ok(http_method)
    }
}

/*
    see 009
*/
#[cfg(test)]
mod tests {

    use super::super::tests::*;
    use super::*;

    #[test]
    fn should_convert_method() {
        use restclient::Method;
        let mut req = default_postman_request_class();
        req.method = Some("GET".to_string());
        assert_eq!(convert_method(&req).unwrap(), Method::Get);
        req.method = Some("POST".to_string());
        assert_eq!(convert_method(&req).unwrap(), Method::Post);
        req.method = Some("PUT".to_string());
        assert_eq!(convert_method(&req).unwrap(), Method::Put);
        req.method = Some("DELETE".to_string());
        assert_eq!(convert_method(&req).unwrap(), Method::Delete);
    }

    #[cfg(test)]
    mod tests {

        use super::super::tests::*;

        #[test]
        #[should_panic(expected = "InvalidPostmanMethod")]
        fn should_fail_to_convert_when_postman_method_is_unknown() {
            let mut req = default_postman_request_class();
            req.method = Some("INVALID".to_string());
            convert_method(&req).unwrap();
        }
    }
}
