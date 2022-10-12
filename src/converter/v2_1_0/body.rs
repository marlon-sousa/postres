use crate::{error::PostresError, postman::v2_1_0, restclient};

/*
    def 017: Generic programming

    Let's talk about generic programming

    If you tried to understand this concept and get either terribly lost between so many brackets or way terrified by some straneous simtax that refused to compile in c++ be assured you are not alone.

    Again, the concept is simple and powerful, but can lead to very complicated code in terms of readability.

    Still, for many reasons, this is one of the features which enables Rust to be so dynamic and powerful in terms of expressiveness.

    Functions usually operate on parameters. If you need to sum two integers you receive both as parameters and return the sum. The function is generic in the sense that it can sum any passed parameters as long as they are integers.

    This is what makes the use of functions be worthwile: because they can operate on any parameter, you don't have to write its body every time when the values of these parameters differ, as long as the parameters are from the agreed types.

    But ... what happens if you need to make the very same operations on parameters of different types?

    Well, you create several functions that can operate on different combinations of types and parameters.

    In languages which allow overloading, you can use the same function name. The compiler will know what function body needs to be called based on the type of the parameters passed.

    In other languages, you will need to specify different names, such as sumInts and sumChars.

    In languages supporting generics, you can make yet another thing: you can use the same function body to act on different combinations of parameters and types.

    In rust this is true as long as the number of parameters are the same.

    Is this way generic?

    Well, let's examine one of our needs here:

    Postman request body can be of several types.

    What defines a body type is a field called type on the body object. Deppending on the value of this field, we have to look at different fields to get the information we need, because they greatly vary.

    For example, if type is "file", we must read the file field to get the information. If it is "urlencoded", we must read the url_encoded field to get the information.

    So, as you might be thinking, every field of the body object is potentially nullable, thus represented by an Option. This is so because, for example, if the type field has the "graphql" value, we expect the graphql field to have the information we need. The other fields, such as file, url_encoded, etc have all the None value, because the body is of type "graphql" and thus we have only graphql information.

    But, because all fields are Option ... nothing prevents the object to specify in the type field that the type is "raw" and provide us with a raw field also with None!

    This shouldn't happen, but deffensive programming requires us to validate it even so, we shall not take external data validity for granted

    This also means that for converting all the possible body types, we will first have to check if the appropriate field is None. If it is None, we must return an error and, if it is not, we must unwrap the field info from inside None and return it to be used as source for the conversion.

    Ooops, doing several times the same thing ... isn't it time for a function? It seems so!

    We, however, have a problem: Each body field is of a different type, and as we know a given function shall receive a list of parameters of fixed type and return a value of fixed type.

    Oh well, let's copy and past several times the same lines and forget about functions then ...

    Well, not exactly, at least when we have generic functions at our disposal to help us with this.

    See, the body of the function is going to be the same:

    We receive as parameters an Option with something inside (if it is the Some variant) of a given type, we don't care about what type it is, and an error. We then verify if this Option is None. If it is, we return an Err variant of a result, with the error we got as parameter.
    If the Option is Some, we return a reference to the value held by the Some variant. Value of what type? Again, for us (the function) this is not important. The only thing we know is that the returned type has to be the same type as the type of the Option we passed as parameter, which makes sense.

    We could write our ideal function as something like this:

    fn assert_field_has_value(field: &Option<SomeThing>, error: PostresError) -> Result<&Something, PostresError> {
        if field.is_none() {
            return Err(error);
        }
        Ok(field.as_ref().unwrap())
    }

    But Something       is not a valid key word. Rust could offer Type1, Type2, and so on, but evem so, you would have several mew language defined words only to represent types that can be anything.

    The solution, is, instead, to declare placeholders for types.

    We do it between handle brackets soon after the function name and before the parameters list. Once declared, they become valid key words in the parameter list and inside the function body.

    fn assert_field_has_value<T>(field: &Option<T>, error: PostresError) -> Result<&T, PostresError> {
        if field.is_none() {
            return Err(error);
        }
        Ok(field.as_ref().unwrap())
    }

    Then, when we call the function passing, for example, an Option<Boolean> type, the body of the function will be compiled to receive an Option<Boolean> and return a reference to the value inside that option if it has the Some variant, or an error if it has the None variant.

    Observe that by:
        1- Declaring T as a placeholder of a type
        2- Specifing T as the value held by the Option parameter
        3- Returning T
    we are letting clear that whatever type is held by the option has to be returned by the function in case of Success

    Generics can be much more complicated than that.

    For example, we might have a requirement to handle a parameter of any type cinse the type conforms to something else.

    A highschool can accept any human as student as long as this human already knows how to read.

    In full object oriented languages, we would make a human, then a student, then a highSchoolStudent, using inheritance. We would then define graduate() method on the HighSchoolStudent.

    The inheritance usage has its drawbacks. Independently of your opinion and personal prefferences, this is not the way Rust handles this kind of thing. Instead, you can define constraints, or limits on which types you accept by requiring a type to implement some trait.

    Comstraining on generics will not be covered in this explanation. Take a look at our function which accepts Options parametrized with any type (without constraints) and returns a result with the Ok variant set to a reference to that same type or an Err variant set with a PostresError

    As always, take a look at the documentation for more details
*/

fn assert_postman_field_has_value<T>(
    field: &Option<T>,
    error: PostresError,
) -> Result<&T, PostresError> {
    let field = field.as_ref().ok_or(error)?;
    Ok(field)
}
/*
    Another way tho think about generics is this: the above function didn't use anything specific about the type represented as T. It checked the state of the Option (the Option is a concret type), returned an error if it was None and unwrapped the T inside the Option amnd returned a reference to it, whatever it is.
    Thus, the function manipulated a given type but didn't go on any specifics about that type (no fields were read, no values were updated, nothing like this), it manipulated the T type in a generic way.
    As we said above, you can restrict the accepted types on a function making sure that they implement some traits.
    Traits are a heavy way of abstraction of behavior on types. Generic functions can only call behavior on types using traits or in cases where the type is transparently used.
    To recall from an example we gave, we can implement the write behavior for humans and makke our function accept any type as long as the type complies to the write trait.
    Our generic function can then call methods (or behavior) defined by the write trait. The function has no idea about how a giben type would write something, but it knows it can safely call the write behavior on that type because there is an agreement between the passed typpe (it implements the write trait) and the function (it accepts types as long as they implement the write trait).
    So, even though our generic function can manipulate types, it can only manipulate them in a way which is previously agreed between it self and the type being manipulated in terms of traits.
    Again, think about it: when you go to a restaurant, you don't (or shouldn't) make assumptions about the waiter serving you, with the exception of they being able to listen to your order and bringing whatever was ordered.
    You don't ask about their families, care if they are man or woman, or what team they are supporters of. You access some agreed behavior on them which configures a relationship between a client and a waiter.
    They, at the other hand, fulfil the requirements and require you (being man or woman) to order clearly and, hopefully, pay the bill at the end. They don't (or also shouldn't at least) make assumptions on you.
    This describes what generics are like.
    Traits can seem to you as interfaces in other languages. The relation is close, though not exactly the same (you can have traits specifying groups of fstateless behavior, much like groups of functions), while most languages require interfaces to be implemented on a stateful structure, called instance in object oriented terminology. While there is much more to be said about them, this is an important enough concept which works closely to generics in rust to be introduced at this moment.
    For more information see 014
*/

/*
    def 020 - pointers, references and lifetimes

    Right, let's talk about references. What are them?

    Pointers. Yes, it is that simple, and so it is not only for rust, but for every other language using references, including c++ and java!

    Ever wondered why java claims that it has no pointers, only references, but dereferences to null cause a NullPointerException? Because references are pointers!

    Now, pointers don't have to be of only one kind. There are several kinds of pointers, including references.

    References are not the same among languages, each one has its specifics, but conceptually references are a kind of pointer.

    Now, what are pointers?

    Pointers are types which hold memory addresses. Integers are types holding a representation of an integer number, floats are types representing a number with floating point specification, and pointers are types which represent memory addresses.

    Pointers are not integers, pointers are not longs, pointers are not numbers. If you think about pointers as being anything else than a type representing memory addresses you will earlier or latter find yourself in trouble, and it is going to be a hard one.

    You can manipulate pointers as numbers, but you would be relying your self on a combination of architecture, operating systems and programming language, so you are better knowing what you are doing.

    Now, if you think about it, pointers are addresses of memory, and in typed languages memory is filled by variables of ... some type.

    Even in weakly typed languages, everything in memory does have a type at run-time, type which has to be checked every time a variable is accessed.

    So, we commonly use pointers (pointers are their selves types) which hold addresses in memory of data of a given type.

    If you're curious, we can use pointers which hold only an address without metadata to inform the compiler about the type of data the address contains. This is a void* in C, an address without information of what kind of data this address contains.

    In c++, if you do it with, say, instances, the destructor won't be called (the compyler doesn't know what a void* points to, only the size of the memory space). In rust, you can play with rraw pointers (*u8), but if somebody asks you, please don't tell you heard about it here.

    In general, you shouldn't play with pointers which represent addresses of memory where data of unknown type is stored.

    Technically, an address is composed as the starting position in memory (base), and an offset (the size the type of variable stored at that address has), plus contextual information of the type of variable stored at that address.
    Not all parts of the address as we described here are stored in memory. Tipically, only the base, the place where the data begins in the memory and the offset (size of the memory this data contains) are stored. The contextual information is information the compiler has at compile time, because you usually declare a pointer and the type of the variable the address where it points is.
    Languages offering what is known as run-time type identification usually store the characteristics of the type the pointer points to alongside with the pointer, so at run-time we can get a pointer with a unknown type and ask for the type it points to, so that we know how to manipulate it.
    For experienced rustaceans or c++ programmers, I let it clear here that I am by no means  trying to explain all the internals and am making several simplifications, because I am more interested in giving a kind of intermediate level conceptual view I have almost never found else where.
    In rust, you can have run-time type identification by using a special type called Any. Otherwise you have no RTTI, following the defaults of only paying expensive prices for things you really need to use.
    Ok, enough conceptual view on pointers by now. Pointers, again, are types holding addresses of variables of given types. Through them you can access (read information) or manipulate (write information) on the variables they point to, or for variables at the addressm they hold.

    But what are pointers used for?

    Well, for pretty much everything. I will not write a whole article on this topic here, but the way I like to think about pointers is like if they were remote controls or access grants.

    Suppose you and your sister are watching TV. You want to change channels. What do you do?

    Perhaps you need to go all the way and press a button on the tv. Or may be you have to bring the tv, with cables and everything else close to you.

    But going back and forth to the tv and back to the sofa is expensive, you spend energy and it is slow. Bringing the tv close to you is also not desirable, but could work.

    Now, you will leave home and your sister wants to control the tv. You can:
    1- Place her near the tv.
    2- Say her where the tv is so that she can go forth and back to control the tv.
    3- Bring the whole tv, with cables and everything else, close to her.

    Again, bringing the tv close to her works. But what if you are in the living roon and she is in the kitchen?

    Everythime one of you want to control the tv, the tv would have to be moved close to you in the living room or close to her in the kitchen.

    Transporting the tv is not cheap either, if more than one person needs to control it!

    So ... having to go to the tv to conmtrol it is not desirable. Neither it is bringing the tv close to you. If you and your sister have to control the tv in the same session, things get even more complicated.

    But ... what if you have a remote control? Well, the tv can be far from you. If you have the remote control you can change channels.

    If you are leaving home, you can go to the kitchen and give the remote control to your sister (being nice to your brothers is always desirable).

    Transporting a remote control is way easier than transporting a tv!

    Further more, you can have two remote controls, one in the living room and one in the kitchen, that are capable of controling the same tv. If you and your sister safely agree and negotiate when changing channels, may be you won't even need to make the efort to give her the remote control wen you leave, because she will have one.

    One of the main uses of a pointer is exactly like if it was a remote control. You have even to point your remote control to the tv before pressing the buttons, which makes this explanation even more realistic!

    There are other use cases for pointers, but specially for references this is by far the main use: when you have your variable (the tv) allocated at one place, and want to have one or more remote control (the references) passed to functions or stored in structs where they (the remote controls) can be used to manipulate the tv without having to either go close to it or bring it close to them.

    Specially in languages using garbage collector, references are the di-facto standard way, many times the only way to access variables. This happens because variables are allocated in a place where the garbage collector has full power to manage them.
    Once functions go out of scope, the remote controls they own are collected, but not the tv. If I get nervous and break my remote control, the tv is not affected. If my parents take the remote control away from me, the tv is still there!

    Right, the same is valid for pointers: the life of pointers, much like the life of remote controls, do not affect the tvs, though the opposit is true: if my tv breaks, all control remotes pointing to it will fail, because even though they are working the tv isn't.

    Remote controls are usually made to manipulate a given equipment. If I try to point my radio remote control at the tv and press buttons, nothing is likely to happen. Understand the control remote as a pointer, and the kind of equipment it is designed to manipulate as the type of variable the address it holds.

    In rust, general pointers are usually created from an allocation result. References, specifically, are not.

    I can create a reference only to something which is already allocated. In our representation, I can give you a remote control for some tv which is already installed and working.

    Also, in rust, the compiler makes sure that no control remotes are keept with anyone once someone takes the tv away.

    If it didn't, we could have one or more people crashing because they are trying to control a tv which does not exist or is not working anymore and expecting results.

    Now, we need to talk about something else related to references, and to pointers in general: ownership.

    The concept of ownership is coded into rust rules, and this is definitely not the case with other languages. To understand ownership, let's back to the remote control concept.

    Rust doesn't like to sponsor fammiliar conflicts. If I and my sister have both remote controls which manipulate the same powered on tv (both have references holding the address of the same variable), what happens if I want to watch news and she wants to watch sports?

    Yeaah, a fight! But rust is pacifist and believes in dialog and negotiation.

    Historically, programmers haven't been good on stablishing dialog and negotiation between several parties interested in manipulating the same piece of data. In their defence (I am one of them), I have to say that negotiating is kit hard to get right, specially when there are lots of
    interested parties on manipulating a given data, and this is probably also true for a family which is watching tv together on a hard's day night :p.

    Knowing that, rust stablished the ownership concept. It works like this: every variable has to have an owner. Too much freedom might be good, but this is not how things work inside a computer program. Again, all variables have to have an owner while they exist.

    Right, I bought the tv and installed it at a given place (I allocated a variable). I am this tv owner. Although several people can watch the tv, I am the only one allowed to change channels.
    If I want, I can give this tv to my sister. Because the tv is close to me (I have no remote control at this time, I am manipulating a tv physically), if I want to give it to my sister there in the kitchen, I have to move the tv close to her.
    If I did so, now she is the tv owner and I have no longer the right to change channels.
    In the other hand, I might decide not to give her the tv. In this case, I keep the tv close to me. I can, however, setup a remote control and give it to her.
    This remote control is small, so by handling it to her I do not spend too much energy, tvs are heavy and they use several cables, moving tvs is not easy.
    My sister can them change channels, but when she is done her remote control is destroied. I have to make an agreement with her that, while she has the remote control and is thus able to control the tv I own, I will not try to change channels.
    So, in practical terms, although she has only a remote control, because of our agreement, she has the total control of the tv, so I effectively borrowed my tv to her, even though she has only a remote control. Once she is done, I can control again my own tv, and nobody else can do it.
    This is important. The only way I can borrow things I own to someone else is by giving them a remote control to the thing I own. If I give them the object I own (not a reference), then I am giving away my ownership of that object.
    No more familliar conflicts because of the tv, if everyone follows that rule (and the compiler obligates us to do so, therefore we always do).

    The story is almost ended. But, before ending, let's use another example:
    I can be working on a report. At a given moment, my coleag might want to complement it.
    I can give her the report, but If I do it     then I will have not the right to keep working after her part is done.
    In the other hand, I can give her access (a reference) to the report. At this time, when she is writting her part, I cannot change the document, otherwise we will interfer with each other.
    But, once the report is done (noone else is working on it), I can give access (references) to several other coleags who will need to read the information to do their work.
    These are also borrows, but they are imutable.
    Rust allow either that several readonly references exist to an owned object or that one mutable reference exists at a given time.
    This way, if I have several people using my report, I need to revoke access of all of them before either changing its contents or letting someone else modify it through a mutable borrow.
    This is not the only way of managing shared data, but usually this is the way we do it with references.
    One last thing: while the report is borrowed to someone mutably or to one or more people imutably, I cannot delete it.
    If I do so, a coleag trying to write something on the report will start writting on nothing, because the document has been deleted. Either this or several coleags will be reading the report when it will just disappear from their screens, letting them with an incomplete picture of what they need to produce their work.
    Again, we have rust here obligating us to be nice with our coleags.
    This last case, where several people might have references to something I own and I want to delete it is usually very complicated to deal with.
    We call references which point to something nolonger available dangling references, and this is an extremely common cause for very hard to discover and dangerous bugs.
    Languages using garbage collector count references pointing to something and only let this thing be deleted when all references have also been deleted.
    This is, however, a slow process and, you like it or not, this is not the way rust works.
    In rust, the compiler scans the source code and make sure that once a variable goes out of scope, there are no references pointing to it. If this is not the case, an error will be issued and you will have to rethink your design.
    As usual, you can also use a kind of reference counted pointer, but this is used when memory is allocated outside of your domains, (we call your domains the stack), in an area called heap. The heap allocations are useful, but slow.
    Some languages, specially those that use garbage collectors, usually only allow allocations in the heap. Rust, in the other hand, by efficiently managing references, allows you to use the stack in a very efficient way.

    References are declared placing an & before the type of data their address point at.

    Now, for lifetimes, let's discuss a little bit more on top of what we have until now.

    Remember: references (or remote controls) can exist for as long time as the things they point to (the tvs) also exist. If I deactivate a tv for which several remote controls are poiting to, these remote controls would be considered as dangling, because the remote controls owners will try to control a tv which is simply not there anymore.

    The rules for making sure references don't dangle are included into the compiler and it deduces, by scanning your code, if you design is dangling references, in which case a nice error is reported.

    We call this code scanner that checks for dangling references at compile time the borrow checker. But there are times when the compiler is not sure about the rules of your game. And when it doesn't know how to interpret your code, it will stop and ask you for more clarifications.

    You usually clarify the rules by using lifetimes

    The below function is interesting. It rreceives a reference to a vector of a generic type.

    It doesn't want to take ownership of the vector, so it receives a read only reference. The only thing this function does is examine if the vector the reference points to is empty.
    If it is empty, the function returns an error. If not, the function returns a reference to the data it received, or in other words another reference to the vector.
    This function is useful because it avoids us the writing of if bla is empty then return an error everywhere. Code reuse, right?
    Oh well, but we have another problem: what error should we return?
    An error with the generic message: " error: list is empty" would solve the problem, but then at runtime we would see that and think ... this could be any list of anything anywhere ... where should we look at first to try to fix this?
    Clearly we need a more descriptive message. Well, you think, that's easy: we create a second parameter with a message, and on each function call we can pass an appropriate one should the error be raised.
    Well, great! This is what I thought also, and in fact this is what we did here.
    But ... our message is also a reference to some text, so that we can define it at the caller without moving it, the same way we also passed a reference to the vector, so that we didn't move it.
    Our function then receives two references: one pointing to the vector and one pointing to the message that should be used in case of the error.
    Memory efficiency, speed, performance, everything is good. What are the lifetime requirements?
    Well, our function needds to be sure that, while it is executing, the vector owned by its caller will not be deleted. This is so because the function accesses the memory of the vector to query its length, to know if it is or is not empty.
    So we know that our vector cannot be dealocated nor can it be changed while the function has a reference to it.
    We also need to make sure that the message it will place in the error is not dealocated, because should the message be needed it has to be available.
    Right, two read only borrows relying on the assumption that neither the vector nor the messages on caller side will disappear.
    This function makes something else though: if the vector is not empty, a reference to it will be wrapped in an Ok variant of a Result enum and this enum will be returned.
    Now, what are the lifetime requirements of this return?
    Well, the returned reference should be accessible also as long as the original vector it points to is accessible.
    What about the message? Well, if the function returned an Ok, the message isn't needed anymore. The message is needed only in case of errors.
    But if the function returned Ok then the reference inside of the Ok variant can be used, so this reference needs to live as much as the original vector does.
    Does it make sense for you? See that we have a rule here: the function needs the vector to be alive while it is running. After the function returns, in case of success, it (the function) goes out of scope, but the reference returned by it inside the Ok variant still requires that the vector is alive, and this returned reference cannot be used after the vector is deleted.
    This rule is deducible by us, but not by the compiler. As the borrow checker sees things, here is how we have the function:
    I have to references as input parameters and one reference is returned.
    I need the programer to tell me the rules of the game: for how much time the returned reference needs to live?

    Well, as said before, lifetimes are the way we use to clarify the rules.

    What we need to state is that the reference returned by the function (if it is returned) has to live as long as the vector referenced in the input parameter

    Lifetimes are named with a ' and a name, without spaces.
    You declare it between handle brackets, exactly like you declare types in generics.
    Then, you anotate references with the defined lifetimes.
    Take a look at what we do in the function below, and then we continue the discussion.
*/
fn assert_has_elements<'a, T>(
    target: &'a Vec<T>,
    field_name: &str,
) -> Result<&'a Vec<T>, PostresError> {
    if target.is_empty() {
        return Err(PostresError::EmptyListOfPostmanItemsError {
            field: field_name.to_string(),
        });
    }
    Ok(target)
}

/*
    In the above function, we specified that we accept as parameters a reference to a vector declared on a given scope. This scope we named as 'a
    We also accept a reference to a str which can be declared at any scope we don't care.
    We also said that we will return a Result with either an Ok variant or an Error variant.
    The Ok variant holds a reference to a vector declared at the same scope as the vector referenced by the first input parameter. How do we know that? Because the reference at the return also have the 'a lifetime, the same lifetime specified at the input parameter.
    It makes sense, because we are returning indeed a reference to the same vector we received a reference to, and these two references (the input and the output) in this case should have the same lifetime, which is live as long as or less than the vector they reference.
    This way, the borrow checker can analyze our code and make sure that the returned reference will not be used after the vector is dealocated.
    The borrow checker  can infer lifetimes for a huge amount of situations. This hasn't always been so. We don't know if inferences for situations like this will ever be authomatically provided. What we know though is that every time rust is in doubt about how to make sure our code isn't misbehaving, it will ask us for further clarifications.
*/

fn convert_body(postman_req: &v2_1_0::RequestClass) -> Result<restclient::Body, PostresError> {
    if postman_req
        .body
        .as_ref()
        .filter(|b| {
            b.disabled.is_none() || matches!(b.disabled, Some(disabled) if disabled == false)
        })
        .is_none()
    {
        return Ok(restclient::Body::Empty);
    }
    let postman_request_body = postman_req.body.as_ref().unwrap();
    if postman_request_body.mode.is_none() {
        // as we have no mode spec, try to return the raw field or an empty request if it also is not provided
        return Ok(http_body_from_raw(&postman_request_body.raw)?);
    }
    let body = match postman_request_body.mode.as_ref().unwrap() {
        v2_1_0::Mode::File => http_body_from_file(&postman_request_body.file)?,
        v2_1_0::Mode::Formdata => http_body_from_form_data(&postman_request_body.formdata)?,
        v2_1_0::Mode::Graphql => http_body_from_graphql(&postman_request_body.graphql)?,
        v2_1_0::Mode::Raw => http_body_from_raw(&postman_request_body.raw)?,
        v2_1_0::Mode::Urlencoded => http_body_from_url_encoded(&postman_request_body.urlencoded)?,
    };
    Ok(body)
}

fn http_body_from_raw(raw: &Option<String>) -> Result<restclient::Body, PostresError> {
    let raw = assert_postman_field_has_value(
        raw,
        PostresError::InvalidPostmanRawSpecification {
            msg: "Raw request body not provided".to_string(),
        },
    )?;
    Ok(restclient::Body::Raw(raw.clone()))
}

fn http_body_from_file(file: &Option<v2_1_0::File>) -> Result<restclient::Body, PostresError> {
    let file = assert_postman_field_has_value(file, PostresError::PostmanFileSpecNotPresent)?;
    /*
        def 016: pattern matching

        This is something that can make you very confused if you have never seen it before.
        Well, the idea is the following: have you ever heard about switch case?
        I guess you did, and pattern matching works *** kind of *** the same in Rust, except that there is much more going on to offer you an incredibly good experience.
        The working schema is simple.
        match something {
            pattern1 => some action,
            pattern2 => another action
        }

        Remember that in rust the match statement is an expression, meaning that it can return something to the outside scope, much like ifs.

        But what are pattern1, pattern2, and other patterns?

        Well, it is a kind of a frame where your matched thing has to fit.
        If it fits, the block is executed. Otherwise, the next frame is tried.

        But there is more to that: matching is exaustive, meaning that the compiler knows and will force you to match against all possibilities provided by the thing you are matching.

        Way generic? Ok, let's take a look at what we need to do here:

        Postman provides two possibilities for a file body: either providing a source file name from which content must be taken or a place where content can be specified inline.

        We are then concerned that:
        1- These two possibilities cannot be both undefined. If we have a file body we either have to have a file name specified or some inline content.
        2- These two possibilities cannot coexist together. If they both exist, what should we do? Use the source file? Use the contents?
        3- If we have the source, use it.
        4- If we have the content, use it.

        One way of achieving that would be using ifs and && operator
        if src.is_none and content.is_none {
            error
        } elseif src.is_some() && content.is_sone() {
            error
        } else if src.is_some() {
            return src.unwrap() // we can use unwrap here because we are sure src has the Some variant
        } else {
            return content.unwrap() // we can use unwrap here because we are sure src has the Some variant
        }

        So many ifs, and logical operators, else ifs, and one else ... and one could easily forget to check one condition, or check it in a wrong way!

        In the other hand, we can use pattern matching. We will match on a tupple containing src and content. Those are two Option enums.
    */
    match (file.src.as_ref(), file.content.as_ref()) {
        /*
            def 016: pattern matching
            if the shape of this tuple matchs a tuple of two Options with the None variant, we will return an error.
        */
        (None, None) => Err(PostresError::invalid_postman_form_for_file_specification("Neither a path to a file nor file contents were provided as part of a postman file form specification")),
        /*
            def 016: pattern matching
            if the shape of this tuple matches a tuple of two Options with the Some variant holding anything (we are not interested in what, the shape just says that there are two Options holding the Some variant), we will return an error.
            Notice we use the _ inside the Some variants, meaning exactly this: The shape is of two Some variants holding whatever thing.
        */
        (Some(_), Some(_)) => Err(PostresError::invalid_postman_form_for_file_specification("Neither a path to a file nor file contents were provided as part of a postman file form specification")),
        /*
            def 016: pattern matching
            if the shape of this tuple matches a tuple of two Options, the first containing the variant Some holding anything inside it and the second containing the variant None, we know that
            The source file information is provided. We know that because the first variable in the match statement is the file.src option.
            Notice that instead of the _ symbol inside the Some we use the name src. This name can be anything amd it means that the shape has to contain two Options, one holding the Some variant with something inside that we are naming src and another containing a None variant.
            Because we named the thing inside the Some variant, we can use this thing after the => in the action block. If we had named it _ instead we would have no way of using it.
        */
        (Some(src), None) => Ok(restclient::Body::FileSource(src.clone())),
        /*
            def 016: pattern matching
            if the shape of this tuple matches a tuple of two Options, the first containing the variant None and the second containing a variant Some holding anything inside it, we know that
            The inline content information is provided. We know that because the second variable in the match statement is the file.content option.
            Notice that instead of the _ symbol inside the Some we use the name content. This name can be anything amd it means that the shape has to contain two Options, one holding the None variant and the second containing a Some variant with something inside that we are naming content.
        */
        (None, Some(content)) => http_body_from_raw(&Some(content.clone())),
        /*
            def 016: pattern matching

            This is a clearer way to express thhese kind of conditional instead of using ifs, mamy times nested, together with logical operators.

            Also, because matches are exaustive, the compiler knows that we are evaluating two options which can contain each one two values, so that we have four possible combinations.

            The compiler will make sure we provide actions to the four possible combinations. Doubt it? Try to comment one of the four possible match arms here (None, None), (Some(_), Some(_)) or the others and see what happens ... yes, the compiler shows clearly that you must handle all possible combinations!

            This both protects you from forgeting to evaluate possible combinations and protects you from not evaluating situations if, for example, the Option enum wins yet another variant. If the Rust team went ahead and changed the Option enum to have a Maybe variant, alongside None and Some, this code would break 
            Because the compiler would say that some more combinations had to be evaluated, now instead of four they would be nine

            But, you are asking, can't I just provide a default condition?

            Yes, and you would loose all that exaustiveness cool bennefits. Even so, if you opted to have a default branch, build with the (_, _) pattern a tuple containing anything and anything, this would be explicite and people would know that by reading your code.

            This only scratches the surface of what pattern matching can do. For other possibilities, search the documentation
        */
    }
}

fn http_body_from_form_data(
    form_data: &Option<Vec<v2_1_0::FormParameter>>,
) -> Result<restclient::Body, PostresError> {
    let form_data = assert_postman_field_has_value(
        form_data,
        PostresError::invalid_postman_form_data_specification("form data not specified")
    )?
    .iter()
    /*
        def 018: destructuring
        In the next line, we have something interesting.
        Our filter is simple: we consider something to be enabled either if the disabled field is None (we assume something is enabled by default) or if the disabled field is present (therefore with the Some variant) and the boolean value inside it is false (something stating that it is not disabled is enabled).
        Please understand that the decision of using a falsi (e.e calling a field disabled instead of enabled) was not mine, it is something from postman specification.
        Right. We then proceed to the filtering closure, receiving each FormParameter in the vector and returning true to keep the form parameter in the resulting vector, false otherwise.
        But, for these filter closure, we only need to look at the disabled field of the FormParameter. We can keep writing param.disabled every where, but this seems a wast of typing. Also, what about if we needed to receive a struct and needed to look only at a sub field of a sub field of it?
        It is desperating to read a.b.c.d several times. Well, destructuring to our help here!
        The concept is this: if you receive a complex object in a function (closures are functions) and only need to look at some parts, you specify this rule in the parameter list and inside the function body you have a way more fluent code experience.
        You can destructure all kinds of complex entities in rust (structs and enums for example), and here is how:
        You place, instead of a simple name for the parameter, a shape composed of the struct field (s) you are interested in, and saying that you don't care for the rest, using the ".." operator.
        Take a look at the closure below and see how we just care about the disabled field.
        One last thing here: Did this recall you of something? If you said pattern matching, you're on a extremely good way.
        In fact, you are matching here on a shape and using what is of interest to you.
        The patterns can also be used in pattern matching. The difference is that the match statement tries to match a series of patterns, while a function parameter with this destructuring requires that the parameter is of a given type, thus if the pattern you're using hhere does not fit on that type you will get an error.
    */
    .filter(|v2_1_0::FormParameter { disabled, .. }| {
        disabled.is_none() || matches!(disabled, Some(d) if *d == false)
    })
    /*
        def 022: inference and the _ symbol
        You will find that the _ symbol is often used in rust when we want to delegate to the compiler something we think it can deduce for it self.
        As practically everything in rust, even when we want the compiler to deduce something, we state this explicitly so that other people reading the code know it.
        In this case, we got an iterator to a vector of FormParameters, filtered some FormParameters out of it and are collecting the results in to a new vector. Vector of what?
        Well, if we got a vector of something, filtered some elements and are collecting back to a vector, because the language is strongly typed, we can assume that the resulting vector will be of the same type that composed the original vector.
        And, if we can assume that, so can the compiler.
        Because of that, you will notice that we specified the type of elements of the vector to be returned as _, because we are sure the compiler can deduce it.
        If the compiler can't or is in doubt, a nice error asking us to define things explicitly will be returned.
    */
    .collect::<Vec<_>>();
    // postman spec is vague on how form data is handled.
    // we handle it as the way it seems it works
    let converted_form_data = assert_has_elements(&form_data, "form data")?
        .iter()
        .map(|postman_form_parameter: &&v2_1_0::FormParameter | -> Result<restclient::FormDataParamSpec, PostresError> {
            // because postman spec says that if no type is provided text should be assumed, we reflect this decision here
            let form_parameter_type = postman_form_parameter
                .form_parameter_type
                .clone()
                .unwrap_or("text".to_string());
            Ok(restclient::FormDataParamSpec {
                content_type: postman_form_parameter.content_type.clone(),
                name: postman_form_parameter.key.clone(),
                // remember that a match block returns the result of the matched expression and we can therefore assign that value to a variable pof rield
                value: match form_parameter_type.as_str() {
                    "text" => restclient::FormParamValue::Text(postman_form_parameter.value.clone().ok_or_else(
                        || {
                            PostresError::invalid_postman_form_data_specification(
                                "value for form data parameter of type text not specified",
                            )
                        },
                    )?),
                    // here we are again assigning the result of a match block to ... what?
                    // to the outer match expression, in case the the matched string has the value "file".
                    // If this happens to be the case, we run a nested match block
                    "file" => match postman_form_parameter.src.as_ref().ok_or_else(|| PostresError::invalid_postman_form_data_specification("src field for parameter of type file not provided"))? {
                        v2_1_0::FormParameterSrcUnion::File(f) => {
                            restclient::FormParamValue::File(vec![f.clone()])
                        }
                        v2_1_0::FormParameterSrcUnion::Files(f) => {
                            restclient::FormParamValue::File(
                                f.iter().map(|f| f.clone()).collect::<Vec<_>>(),
                            )
                        }
                    },
                    /*
                        def 023: early return with the ? operator
                        we still need to reflect a little bit more on the line below
                        what is this?
                        From the beginning:
                        First of all, remember that pattern matching is exaustive.
                        When we are matching against enums, we know all possibilities (enums are exactly that, a list of all possible values for a given type)
                        When we are matching against strings or other values, the possibilities are endless.
                        Rust, therefore, forces us to provide a catch all clause, to make sure we are handling unexpected values (or may be they are expected but we don't care, we handle all of them the same way)
                        As you probably are aware (see 016), we can use a name (whatever name) to match everything.
                        The other name is good enough. We could have used the _ symbol, but we need to print the value (whatever it is) in the error message, and this is why we are binding it with a name.
                        Now, you might notice that our catch all branch is returning an Err variant.
                        Not only is it returning this variant, it is also placing a question mark (?) soon after the variant ... it makes no sense at all, right?
                        What is going on exactly here?
                        Well, let's discuss first what our match branches are returning.
                        In case the value is "text", the branch returns a restclient::FormParamValue enum.
                        In case the value is "file", we return whatever an inner match returns. What does it return then?
                        The inner match block matches a field of type FormParameterSrcUnion.
                        This is an enum with two possible values: File and Files.
                        In case the field is of the variant File, the inner match block returns a restclient::FormParamValue enum
                        In case the field is of the variant Files, the inner match block also returns a restclient::FormParamValue enum.
                        So, if we have "text", we return a restclient::FormParamValue enum. If we have "file", we run a match block which returns, regardless of the possible results, also a restclient::FormParamValue enum.
                        All great, untill we reach this third branch, which returns ... an Err. An Err of what?
                        Rust is a strongly typed language, so if a block declares it returns a restclient::FormParamValue enum, it should return a restclient::FormParamValue enum. The restclient::FormParamValue enum does not have an Err variant. Rust is not a strongly typed language! I have been sheated!
                        Ooops, not exactly. The strong typing is still alive and kicking. The seeminglyculprit is that inocent question mark after the Err variant.
                        But, far from being a culprit, this is likely to become one of our best friends, as long as we can understand what it does.
                        And here is what it does: whenever a line is marked with a question mark, this line causes a return if:
                        1- this line evaluates to a std::Result enum.
                        2- the std::Result enum contains the Err variant.
                        3- the function this line belongs to also returns a std::Result Enum.
                        4- The generic type the Err variant on this line has is compatible with the generic type of the Err variant the function this line belongs to returns.

                        Let's check if this applies to our case:
                        this line evaluates to a std::Result enum: checked. This line returns a std::Result enum with the Err variant.
                        the std::Result enum contains the Err variant: Checked, we have just discussed this above.
                        the function this line belongs to also returns a std::Result Enum: checked, this line belongs to a closure. The closure returns a std::Result enum as you can see in its definition line.
                        The generic type the Err variant on this line has is compatible with the generic type of the Err variant the function this line belongs to returns: Checked, the line below returns a Err variant with a PostresError which is of the same type  parametrized in the Result the function it belongs to (the closure) returns.

                        It turns out that by using the question mark we can return early and let our caller deal with the error.
                        If the Result enum has the Ok variant, the question mark operator unwraps the type held by the Ok variant, avoiding a call to .unwrap() and places the success value right where it is expected to be.
                        But, you might ask, the match block does not return an Ok variant.
                        In fact, it does not. The match block does not need to return an ok variant, the match block is free to return anything it wants.
                        The function the match block is within needs to return a std::Result enum, this is the important part here.
                        The question mark will cause an early return at any point inside the function with the Err variant it has.
                        If you look at the closure, you wwill see that it wraps its return in a Ok(...) block, effectively returning either an Ok if everything is successful or an Err if anything fails.
                        
                        This has important consequences in the way you program:
                        In one hand, if your errors are all compatible (this is why most part of rustaceans will describe their errors using a specific enum, such as PostresError), you can decide when (at what level) you want to handle them.
                        This avoid repeating if err != nil { ... } every where, making for a good and fluent code read.
                        In the other hand, the concept of throwing exceptions doesn't apply either. You are still required to handle your errors, you will have to match the return of some function somewhere and react if it is an Err variant.
                        The types of the errors are coherent and compatible throughout the function call stack, and there is no possibility of a function declaring it returns a type end up throwing a different error catched by some other code you don't have idea about where it is.
                        This is a good example of the intermediate approach (neither that explicit mnor that implicit) that rust takes to handle common programming needs.

                        As a last topic, I have anotated the parameter and the return type of the closure to make it easier for uyou to understand what it returns.
                        The compiler would be able to understand these types without explicit annotations, and this is what you are likely to find when reading code.
                        We could write the closure definition like this:

                        |postman_form_parameter|  { ... }
                        Because of the Ok(...) the compiler know that this returns an Result with the Ok variant parametrised with what the Ok() block returns and the Err variant is parametrized with PostresError.
                        Because we are interating in a vector of v2_1_0::FormParameter the compiler also knows that each element (thus the closure parameter) is of this type.

                        One of the challenging parts of reading closures many times is figuring out what parameters they take and what they return.
                        In general, if in doubt, take a look at the constraints of the types the closure declares as parameters and return type.
                        For more information, see 017 and see 022
                    */
                    other => Err(PostresError::invalid_postman_form_data_specification(
                        &format!("form data parameter of type {other} not supported"),
                    ))?,
                },
            })
        })
        /*
            def 023: early return with the ? operator
            Here is another example of early return:
            everytime we specify that we are collecting into a Result parametrized with a collection on Ok variant and an Err variant with a known error, the collector will iterate through the chain of iterators and expect a std::Result with something or the parametrized error.
            If all iterated elements are of variant Ok, the requested collection is returned in the Ok variant of the Result.
            If any of the iterated elements return Err, the collector will return an Err variant with the data returned in the Err variant of the element.
            We, therefore, use the question mark to return early if any item of the vector returned an Error.
            If all elements are successful, the question mark operator unwraps the result from Ok and assigns the collected vector into the converted_form_data variable, that we return as the data the restclient::Body::FormData requires, all inside an Ok variant, because the http_body_from_form_data function also returns a std::Result enum.
        */        
        .collect::<Result<Vec<_>, PostresError>>()?;

    Ok(restclient::Body::FormData(converted_form_data))
}

fn http_body_from_graphql(
    graphql: &Option<v2_1_0::Graphql>,
) -> Result<restclient::Body, PostresError> {
    /*
        def 015: The Option type (no more nulls in safe code)
        One of rust strenghts is the lack of null representations, at least in safe code.
        But, you might  be thinking, programming languages represent concepts and the concept of null defenitely exists. Does Rust completely ignore this concept?
        Of course not. instead, Rust provides an enum called option, with tho variants: Some(x) representing instances where we do have x at hand or None representing situations when x is not available
        If you think about it, we use null lots of times to represent situations where what we requested is not available, for whatever reason.
        Right, you are thinking now, we just changed names from NULL to None, what a nice thing. We still have null, named as another thing!
        Well, remember that Rust enums are more like unions. You can use them as simple tagged options, but you can also attach data on variants, data which does not have to be of the same type for each of the union elements.
        Remember also that Rust forces you, by default, to handle all variantes of an enum when you are reading the data.
        So the difference here is that you have no way of placing a null value where a non null value is expected, like you can do with pointers and references in other languages.
        Instead, if you want to receive a potential nullable parameter in your function, you have to receive an Option<X> and inside the function handle the two possibilities: when x of type X is available and when it is not.
        For that, you can use a variety of strategies. You can also recall your other languages and force a *** kind of *** null pointer exception by calling without any further checking the .unwrap() method on the Option.
        This will return the X contained in Some if the enum has the Some(X) variant or panic the process if it is None.
        But, even if you do so (hmm, you really shouldn't most part of the times), you are explicitely doing that and letting everyone reading your code know that this is being done.

        Well, There is more on the Option enum, several other pretty sweet things, and this is why I am writting this text here.
        Here is what we need to do:
        Postman collection has informed us that the body of this request is of type graphql. Even so, the schema returns the graphql information as an Option (potentially null), and there is nothing we can do about it.

        So, we need to:
        1- Make sure our graphql data is not null, if it is we return an error (under our control, we do not panic, we just return a known error)
        For that, we use the great ok_or method, which converts the Some enum in to another one, called Result.
        The reasoning is simple: if the option contains the variant Some(x), this method will return the Ok(x) variant of the enum Result.
        If the Option contains the None variant, this method wwill return the Err variant of the Result Enum, with the Error specified in the map_err parameter as the error.
        Finally, to access the unwrapped value in the Some variant, we use the ? operator to cause an early return with the specified error, cinse its type is compatible with the error this function returns
    */
    let graphql = graphql
        .as_ref()
        .ok_or(PostresError::PostmanGraphqlSpecNotPresent)?;
    /*
        def 015: The Option type (no more nulls in safe code)
        I hope you can see how elegant it is to handle errors like this. In my opinion, better than if err return something all around or even worse just getting a null pointer exception for free.
        But there is more to the party:

        2- Now that we are sure our data is not null, we need to extract two fields from it. We know this by reverse engineering the way postman creates the graphql body.
        This is a json object with two fields: one called query, of type string, containing the query or mutation spec.
        The other is also a json string called variables, which can be null and contains also a string with the list of variables.
        For the query field, it cannot be null. If it is, then we have an invalid request body, because a graphql body has to have at least one query or mutation.
        The way we will extract information of a JSON object at runtime is by using the pointer method on the object representing a JSON value.
        This method takes a kind of json path and returns a potentially null JSON object corresponding to the json query like specified.
        If you think that this will return an Option<serde_json::value> you are right.
        Ok, let's talk first about the spec part:
        After applying the .pointer() method, we will have to return early if None comes out, meaning that the query field is not present in this JSON fragment.
        But if we got Some instead, what is wrapped is yet another serde_json::Value, meaning that we know that a field query exists and that it points to some other JSON fragment.
        We know that this fragment must be of type String, as per specification. So we will need to call the .as_str() method which, again, will either return a Some containing the string or a None if this value is not of type string.
        Hmm, this is starting to get boring. So many .map_err on our way, this is not fluent. Do we have other options?
        Yes, we do, proudly provided by several other methods of the Option enum, to allow us to process a data in a chained way (very functional), like if we were piping.
        We have seen other chained processing strategy before, when we talked about iterators (see 010)
        In fact, you will notice that several transformers, such as map, flatten and filter are available also for options.
        we will use another transformer, called and_then.
        The and_then method transforms an Option<X> in an Option<Y>, by applying a function.
        If the Option<X> is None, None is returned.
        If the Option is Some, the unwrapped value is passed to the function, which has to return another Option.
        This feets our needs: we will call the pointer() method which returns an Option. If this is None, we want to have None as our process result.
        If it is Some(JsonObjectWithUnknownType), we want to get that object in a function and call the .as_str() on it. The as_str() method returns itself an Option, so it is perfect.
        At the end of this phase, we will have a None if either the pointer() returns None or if the as_str() called on the Some(obj) returned by pointer() returns None. If both returns Some, we will have a nice Some(&str) containing the string corresponding to the query field.
        But the spec requires a String, not a &str. So we will use a map, which gets an Option and if it is None the None is returned, otherwize a function taking the unwrapped value contained in Some is called. This function needs to transform that value. In our case, it applies the .to_string on the value, transforming it from &str into String.
        The map then wraps this transformed value a Some variant.
        At this phase, we will have either a None if pointer() or as_str() returned None or a Some(String) constructed over the Some(&str) constructed over the Some(JsonObject) generated, respectively, by the map, and_then and pointer methods.
        Finally, we will call the ok_or_else method to either return early with an error or get the unwrapped String for use latter.
        This chained processing could go on and on, transforming the Option until we get what we need at the end.

        As for the variables field, we will do exactly the same, the only different thing is we will not call ok_or_else, because we are ok if the variables field is None.

        Before ending this long block, we will talk about one last thing: You might be confused about the difference between and_then and the map transformers.
        It is simple: in the and_then, the transformer function (or closure) is responsible to wrap the transformed value in an Option.
        In the map, the transformer (or closure) function returns a value. This value is then wrapped by the map function as Some(val) before map returns.
        Therefore, in a map, if the Option the map is called on is Some, a Some(transformed value) will be returned. If the source Option is Some, the returned Option will be Some.
        In the and_then, however, the transformer function can either return Some(transformedVal) or None, so that:
        1- If the Option and_then is called on is None, the resultant Option will be None.
        2- If the Option and_then is called on is Some, the resultant option can either be Some(transformedVal) or None, deppending on what the transformer function (or closure) decides.
    */
    let spec = graphql
        .pointer("/query")
        .and_then(|q| q.as_str())
        .map(|q| q.to_string())
        .ok_or_else(|| PostresError::InvalidPostmanGraphqlSpecification {
            msg: "Could not extract query of mutation from graphql body".to_string(),
        })?;
    let variables = graphql
        .pointer("/variables")
        .and_then(|q| q.as_str())
        .map(|q| q.to_string());
    Ok(restclient::Body::Graphql(restclient::GraphqlSpec {
        spec,
        variables,
    }))
}

fn http_body_from_url_encoded(
    parameters: &Option<Vec<v2_1_0::UrlEncodedParameter>>,
) -> Result<restclient::Body, PostresError> {
    let parameters =
        assert_postman_field_has_value(parameters, PostresError::PostmanUrlEncodedSpecNotPresent)?
            .iter()
            .filter(|param| {
                param.disabled.is_none() || matches!(param.disabled, Some(p) if p == false)
            })
            .collect::<Vec<&v2_1_0::UrlEncodedParameter>>();
    if parameters.is_empty() {
        return Err(PostresError::PostmanUrlEncodedSpecNotPresent);
    }
    let mut params = vec![];
    for postmanParam in parameters {
        params.push(restclient::QueryParam::new(
            postmanParam.key.clone(),
            postmanParam.value.as_ref().cloned().unwrap_or_default(),
        ));
    }
    Ok(restclient::Body::UrlEncoded(params))
}

/*
    see 009
*/
#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::super::tests::*;
    use super::*;

    #[test]
    fn should_return_empty_request_body_when_postman_request_body_is_not_provided() {
        let postman_request = default_postman_request_class();
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(res, restclient::Body::Empty);
    }

    #[test]
    fn should_return_empty_request_body_when_postman_request_body_is_disabled() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            disabled: Some(true),
            ..Default::default()
        });
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(res, restclient::Body::Empty);
    }

    #[test]
    fn should_return_raw_body_when_postman_body_mod_is_not_provided() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            raw: Some("request_body".to_string()),
            ..Default::default()
        });
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(res, restclient::Body::Raw("request_body".to_string()));
    }

    #[test]
    fn should_return_raw_body_when_postman_body_mod_is_raw() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Raw),
            raw: Some("request_body".to_string()),
            ..Default::default()
        });
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(res, restclient::Body::Raw("request_body".to_string()));
    }

    #[test]
    fn should_return_error_when_postman_body_mod_is_raw_but_raw_body_is_not_provided() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Raw),
            raw: None,
            ..Default::default()
        });
        let res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_convert_to_graphql_body_with_variables() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Graphql),
            graphql: Some(json!({
                "query": "query(2, 2) { result }",
                "variables": "a: 1"
            })),
            ..Default::default()
        });
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(
            res,
            restclient::Body::Graphql(restclient::GraphqlSpec {
                spec: "query(2, 2) { result }".to_string(),
                variables: Some("a: 1".to_string()),
            })
        );
    }

    #[test]
    fn should_convert_to_graphql_body_without_variables() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Graphql),
            graphql: Some(json!({
                "query": "query(2, 2) { result }",
            })),
            ..Default::default()
        });
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(
            res,
            restclient::Body::Graphql(restclient::GraphqlSpec {
                spec: "query(2, 2) { result }".to_string(),
                variables: None,
            })
        );
    }

    #[test]
    fn should_return_error_when_graphql_body_has_no_query_spec() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Graphql),
            graphql: Some(json!({})),
            ..Default::default()
        });
        let res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_return_error_when_graphql_body_is_invalid() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Graphql),
            graphql: Some(json!({"query": 1})),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_return_error_wwhen_body_is_url_encoded_but_no_url_encoded_is_provided() {
        let mut postman_request = default_postman_request_class();
        // no url encoded field
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Urlencoded),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request);
        assert!(res.is_err());
        // url encoded parameter comtainimng empty list
        postman_request
            .body
            .as_mut()
            .unwrap()
            .urlencoded
            .insert(vec![]);
        res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_return_error_wwhen_body_is_url_encoded_but_all_url_encoded_parameters_are_disabled() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Urlencoded),
            urlencoded: Some(vec![
                v2_1_0::UrlEncodedParameter {
                    disabled: Some(true),
                    ..Default::default()
                },
                v2_1_0::UrlEncodedParameter {
                    disabled: Some(true),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_convert_url_emcode_body() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Urlencoded),
            urlencoded: Some(vec![
                v2_1_0::UrlEncodedParameter {
                    disabled: Some(true),
                    ..Default::default()
                },
                v2_1_0::UrlEncodedParameter {
                    key: "abc".to_string(),
                    value: Some("def".to_string()),
                    ..Default::default()
                },
                v2_1_0::UrlEncodedParameter {
                    key: "ghj".to_string(),
                    ..Default::default()
                },
                v2_1_0::UrlEncodedParameter {
                    key: "pqr".to_string(),
                    value: Some("rsp".to_string()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        });
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(
            res,
            restclient::Body::UrlEncoded(vec![
                restclient::QueryParam::new("abc", "def"),
                restclient::QueryParam::new("ghj", ""),
                restclient::QueryParam::new("pqr", "rsp"),
            ])
        );
    }

    #[test]
    fn should_return_error_when_file_spec_is_not_provided_for_postman_request_body_of_file_type() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::File),
            ..Default::default()
        });
        let res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_return_error_when_neither_src_nor_content_are_provided_for_http_request_file_body() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::File),
            file: Some(v2_1_0::File::default()),
            ..Default::default()
        });
        let res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_return_error_when_both_src_and_content_are_provided_for_http_request_file_body() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::File),
            file: Some(v2_1_0::File {
                content: Some("content".to_string()),
                src: Some("file source".to_string()),
            }),
            ..Default::default()
        });
        let res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_generate_http_request_body_of_file_type_when_postman_request_body_of_file_type_points_to_a_source_file(
    ) {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::File),
            file: Some(v2_1_0::File {
                src: Some("file source".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        });
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(res, restclient::Body::FileSource("file source".to_string()));
    }

    #[test]
    fn should_generate_http_request_body_of_raw_type_when_postman_request_body_of_file_type_points_to_a_in_place_file_content(
    ) {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::File),
            file: Some(v2_1_0::File {
                content: Some("content".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        });
        let res = convert_body(&postman_request).unwrap();
        assert_eq!(res, restclient::Body::Raw("content".to_string()));
    }

    #[test]
    fn should_return_error_when_body_is_form_data_but_no_form_data_parameters_are_provided() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request);
        assert!(res.is_err());
        postman_request.body.as_mut().unwrap().formdata.insert(vec![]);
        res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_return_error_when_body_is_form_data_but_all_form_data_parameters_are_disabled() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            formdata: Some(vec![
                v2_1_0::FormParameter {
                    disabled: Some(true),
                    ..Default::default()
                },
                v2_1_0::FormParameter{
                    disabled: Some(true),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]    
    fn should_convert_to_rest_client_body_of_type_formdata_with_text_content_when_postman_form_data_parameter_does_not_specify_parameter_type() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            formdata: Some(vec![
                v2_1_0::FormParameter {
                    key: "param1".to_string(),
                    value: Some("value1".to_string()),
                    ..Default::default()
                }
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request).unwrap();        
        assert_eq!(res, restclient::Body::FormData(vec![
            restclient::FormDataParamSpec { content_type: None, name: "param1".to_string(), value: restclient::FormParamValue::Text("value1".to_string()) }
        ]))
    }

    #[test]    
    fn should_convert_to_rest_client_body_of_type_formdata_with_text_content_when_postman_form_data_parameter_is_of_text_type() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            formdata: Some(vec![
                v2_1_0::FormParameter {
                    form_parameter_type: Some("text".to_string()),
                    key: "param1".to_string(),
                    value: Some("value1".to_string()),
                    ..Default::default()
                }
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request).unwrap();        
        assert_eq!(res, restclient::Body::FormData(vec![
            restclient::FormDataParamSpec { content_type: None, name: "param1".to_string(), value: restclient::FormParamValue::Text("value1".to_string()) }
        ]))
    }    

    #[test]
    fn should_return_error_when_body_is_form_data_of_type_text_but_no_value_is_provided() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            formdata: Some(vec![
                v2_1_0::FormParameter {
                    key: "param1".to_string(),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request);
        assert!(res.is_err());
        postman_request.body.as_mut().unwrap().formdata.insert(vec![
                v2_1_0::FormParameter {
                    form_parameter_type: Some("text".to_string()),
                    key: "param1".to_string(),
                    ..Default::default()
                },
            ]);
        res = convert_body(&postman_request);
        assert!(res.is_err());
    }


    #[test]    
    fn should_convert_to_rest_client_body_of_type_formdata_with_file_content_when_postman_form_data_parameter_is_of_file_type_with_one_file() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            formdata: Some(vec![
                v2_1_0::FormParameter {
                    form_parameter_type: Some("file".to_string()),
                    key: "param1".to_string(),
                    src: Some(v2_1_0::FormParameterSrcUnion::File("file1".to_string())),
                    ..Default::default()
                }
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request).unwrap();        
        assert_eq!(res, restclient::Body::FormData(vec![
            restclient::FormDataParamSpec { content_type: None, name: "param1".to_string(), value: restclient::FormParamValue::File(vec!["file1".to_string()]) }
        ]))
    }    

    #[test]    
    fn should_convert_to_rest_client_body_of_type_formdata_with_file_content_when_postman_form_data_parameter_is_of_file_type_with_more_than_one_file() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            formdata: Some(vec![
                v2_1_0::FormParameter {
                    form_parameter_type: Some("file".to_string()),
                    key: "param1".to_string(),
                    src: Some(v2_1_0::FormParameterSrcUnion::Files(vec!["file1".to_string(), "file2".to_string()])),
                    ..Default::default()
                }
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request).unwrap();        
        assert_eq!(res, restclient::Body::FormData(vec![
            restclient::FormDataParamSpec { content_type: None, name: "param1".to_string(), value: restclient::FormParamValue::File(vec!["file1".to_string(), "file2".to_string()]) }
        ]))
    }    

    #[test]
    fn should_return_error_when_body_is_form_data_of_type_file_but_no_src_is_provided() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            formdata: Some(vec![
                v2_1_0::FormParameter {
                    form_parameter_type: Some("file".to_string()),
                    key: "param1".to_string(),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request);
        assert!(res.is_err());
    }

    #[test]
    fn should_return_error_if_there_are_mixed_valid_and_invalid_postman_formdata_parameters() {
        let mut postman_request = default_postman_request_class();
        postman_request.body = Some(v2_1_0::Body {
            mode: Some(v2_1_0::Mode::Formdata),
            formdata: Some(vec![
                v2_1_0::FormParameter {
                    key: "param1".to_string(),
                    value: Some("value1".to_string()),
                    ..Default::default()
                },
                v2_1_0::FormParameter {
                    key: "param1".to_string(),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        });
        let mut res = convert_body(&postman_request);
        assert!(res.is_err());        
    }
}
