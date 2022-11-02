use std::{
    cell::{self, RefCell},
    ops::Deref,
    rc::{Rc, Weak},
};

/*
    024: is rust really easy or you're sheating me? (a quick reflection on programming)

    Well, this is kit phylosophical.
    First of all, programming is hard. It might appear to be easy, but, again, this is hard.
    This happens because the computer is a limited machine, with a limited view of the world, a world which we, programmers, are trying to represent in our programs.
    Whem something is complex, one measure we can take to represent it is to break this thing, whatever it is, in a group of more streight other things.
    Take this example:
    Suppose we need to teach a given entity to multiply two numbers. Such entity doesn't know, nor it will ever be capable of knowing, the concept of multiplication.
    What can we do then? Give up? Well, not yet. Let's start by asking this entity what are the things it knows to do.
    Entity says: "the only things I know how to do are to store values, to retrieve values, to compare mnumbers, to sum, to repeat, to stop repeating and to count"
    Good. Now, as programmers, we need to decompose the multiplication, a concept we know but the entity does not, in things the entity knows.
    It turns out that multiplying is just a matters of summing the first operand as many times as the second operand specifies, ence 2 * 3 = sum of 2 for 3 times.
    We say: entity perform the following

    store 0 in a label called acumulator, and 1 in a label called control.
    Now, start repeating this:
        sum the value of the label acumulator with the first operand, and store the result in the label acumulator
        compare the value of label control with the second operand. If greater or equals to the second operand, stop repeating
        sum 1 to the value of the label control, and store the result in the label control

    And, after this processing, we have the multiplication result in the acumulator label!

    Now, we can call this group of instructions "multiply". Once we do it, some other programmer can call res = multiply 2, 3 ... and result will be 6.

    From this programmer point of view, the entity "knows how to multiply" , though this is not strictly true: the original coded operations the entity support are suming, repeating, comparing, stop repeating and counting.

    This programmer is, thus, using what we call an abstraction: they know that by calling multiply and passing two operamds a multiplication result is emited, though they don't know neither if the entity already supports it as an encoded operation or if this operation is programmed by someone else nor, in this case, what is needed to perform a multiplication.

    Our programmer doesn't have to know math, they only need to know what a multiplication is (not how it is achieved) and use it appropriately.

    This might appears to be a very obvious thing, but we tend to forget what abstractions are. We also tend to forget that every time something "just works", it is in reality using lots of abstractions behind the cenes.

    Again, this reflects reality, as is the case with almost all things in computing: when you go to a restaurant and order a spaghetti, you don't know how that meal is being prepared. Is it pre cooked? Are the spaghettis hand made by the restaurant or bought from soneone else?
    How many time are them being cooked? For how many cookers?
    These are questions that should interest cookers or people wanting to become a chief, but most part of people just want to enjoy a dinner.
    Notice, though, that there might be super qualified chieves who know everything about how to prepare a good spaghetti. They might even produce their spaghettis manually, because a more abstract way (which would be buying them from someone else) might not render the quality or characteristics they want.
    These super qualified chieves use, therefore, water and wheat floor. But, if you ask them, few will know exactly where the wheat they use comes from, how to grow wheat, how to prepare wheat floor.
    Similarly, few of them will know where the water they use comes from or what kind of treatment is applied to the water to ensure it is ready for use by humans. And, even so, their spaghetti will be delicious.
    The wheat producers, in the other hand, know how to grow wheat, but few of them will know how all the products, machines and strategies they use to grow that weat work. This is a knowledge spread on several other areas.
    Neiter you, nor the chief, nor the farmers are lacking competence: They are just being parts of several layers of abstraction which together compose something that does not exist in the nature, the spaghetti.
    Now, stop and think about this for a while.
    In programming, we have several abstraction layers. The lowest the layer the closest you are to hardware and, therefore, the more limited are the available operations. In the other hand, the higher you are in the layer, the farer you are from the hardware and, because of all the abstractions already in place, the closer you are to the view of the world you need to represent.

    With all this said, this is a module where we will go a level lower than the rest of this project and, as a result, we will create an abstraction, representing a tree.
    As far as the rest of the project is concerned, a nice tree will be ready for use, in the same abstraction level of the rest of the other abstractions we use.

    You don't need to read this module to follow the rest of the project. You don't need to understand what is going on here to use rust for a normal program like this project is. We could just have choosen a crate library (an abstraction layer) for obtaining a tree we could use.
    And this is where rust is different from other languages: must high level languages are not able to access a lower level, where direct memory access and other operating system functions are available.
    In fact, if you are using python or javascript, most low level structures are written in C, and access to their functionality are exported to these languages.
    In the other hand, most part of low level languages aren't able to offer a nice experience for high level programmers, allowing them to concentrate on what they need instead of on low level aspects, even with abstractions.
    Rust, with its modular system and a vast number of high quality libraries covering almost anything, can offer the best of both worlds: if you are a high level programmer, you might very wel spend months before having to deal with low level stuff, while if you are a low level programmer you can make what you need and isolate it in a module, from where you or other programmers can use it in a higher level program, exactly what we are just doing now.
    Before going forward, one last thing: C and C++ have been the di-facto standard programming languages from where higher level languages are written, and therefore the language where low level functionality exposed to these languages are also written.
    We already said this, but here it goes again: Rust, being as low level as C is, offers a way more safe alternative for writting code, the high level code we are writting here as well as the lower level code being written by so many other programmers.
    If you're using a C based language, the low level functionality you're using written in C is exposed to all the risks any software written in that language is also exposed, even using a higher level language.
    If you're using high level rust which uses lower level rust as functionality provider, chances are tat your code and the code you are using are all safer.
*/

/*
    025: trees

    As we usually do, we will discuss what this module is and how it will be used

    Trees are data structures representing hierarchy.

    The idea is simple: we have an element which can hold a value and can also have, under it, other elements of the same type.
    Each element can also have a parent element associated, specifying that it is under that element.

    I can't believe you didn't yet imagine situations in real life where this is being used. I will help you:

    A corporation structure of workers: First, the owner. Under them, the vici-presidents, all reporting to the owner.
    Under each vici-president, one or more directors.
    Under each director, one or more senior managers.
    Under each senior manager, one or more managers.
    Under each manager, one or more ... whatever kind of operational role the company has.

    Another example?
    The root drive directory. Under the root directory, directories. Under each directory, other directories.

    Yet another example?

    The way we classify places in countries:
    First, the country. Then, under the country, the states or provinces. Then, under each state or province, the cities. Then, under each city, the zones or neighborwoods.

    We call the elements which have other elements under them nodes.
    We call elements with no children leafs.
    Each element must know the element it responds to (the parent), the elements responding to it (its children), and the previous and next elements at the same level it is (their ciblings)
    We call the only element which have no parent the root node.

    Trees are structures very good for grouping data in a given order and for applying or finding elements from a given start point.

    There are other structures, such as lists and maps, all of them with a very main challenge: computers know nothing about structures. Trees, lists or maps have no special meaning for computers.

    But, at the other hand, these are very important representations for programmers, because they can be used to represent any kind of thing.

    We have seeing here how trees are suitable for representing hierarchical content. Maps are used to represent dictionaries, while lists can be used to represent anything which can be listed.

    Data structures are a part of computing science and I won't spend much time talking about them here, there are whole books covering that topic.

    I will, however, say one more thing about structs like trees:

    Did you realize that these structs have meaning only because they hold (or contain) data?

    A list has to list ... something. A tree represents hierarchy of ... something.

    This is why we group most of these sctructs into a group called containers. These are data structures which contain data.

    Now, we will discuss our tree implementation:

    Our main element is a Node. A Node has to know its parent node, its children amd the node before it and the node after it in the same hierarchycal level.

    The node has also to hold the element value.

    Take a look at the struct below, and we will continue the discussion.
*/

/*
    031 - pub versus pub(crate)

    Although we are building a library, we don't want this tree to be used outside the library it self.

    The library goal is to provide postman to rest client conversion, not a tree container.

    In the other hand, if we don't make this struct pub, it will be accessible only from within this module.

    What is the solution?

    Mark it as pub(crate). This means that, from the crate it self, this is considered public. But if someone is using this crate, this struct isn't accessible.
*/
pub(crate) struct Node<T> {
    inner: Rc<RefCell<InnerNode<T>>>,
}

/*
    026: An abstraction example

    What? You're angry with me because clearly the Node structure we just read don't have nothing like what a tree should have!

    Well, this is not my fault: This is so because, remember, computers have no idea about what a tree is.

    Take a look at the impl block below. You don't need to read the implementation now, just go ahead and read the method names.
    Right, the methods appear to cover what a tree should offer. Even so, the struct have nothing like the data needed to offer this functionality.

    "The struct has, instead, only one field called inner, of a type I have no idea about what it is", you are thinking.

    Exactly! And its name should give us a hint of what it is used for: this is a inner data, of a lower level than what we wanted to expose to our users!

    Now, if you read the implementation, you will notice that each method here manipulates this inner data in a strange way, delegating calls to whatever this inner data has and doing several other things that would appear strange for must high level programmers.

    And this is how we provide an abstraction level. To have an idea about how we use our Node structure, take a look at the tests at the end of this module.
*/

/*
    027: the stack and the heap

    In order to understand what is going on here, we will have to discuss conceptually the stack and the heap.

    We talked quickly about this when we discussed references (see 020), but let's put things in their places.

    When your program begins, it does so at a function.
    This function must let clear how much memory it will need. This has to happen and this is static, meaning that the function must inform exactly how much memory it needs.
    When this function calls another function, the called function reports how much memory it will need. This is static (at compile time). The memory is allocated and the function runs.
    During the function run, only the memory allocated for that function is accessible. When the function returns, all that memory is dealocated and the caller function is back at its own memory. All data the called function created disappears.
    This is the way most computer programs are laid out. Just keep in mind that programs work like this.

    There are reasons for this layout. We call the memory your program says it needs the stack.

    Stack allocations and dealocations are fast. Basically, you're working on a memory block you asked for to the operating system and thus you own that memory.

    Stack allocations also provide to each function a kind of virtual memory space, in the sense that the function has access only to its memory and is sure that, once a return occurs, all that memory will be inaccessible.

    Let's, again, imagine the restaurant.

    Suppose that you are now the function in execution. When you order a meal, you call the prepare meal function.

    While this function is executing, you are paused waiting for the meal. It is as if you and everything around (the tables, the people, all the rest) were imediately suspended, put to sleep, as if the cameras cut to another environment, the prepareMeal environment.

    The prepareMeal environment is allocated: cookers, oven, spoons, pans, whatever is needed to make the work.

    The meal is prepared and put in a special area. Then, the prepareEnvironment disappears, is deallocated, is cleaned.

    Nothing on that environment is accessible any more, it just stops existing.

    Your environment is tthen restaured, exactly as it was before the function call, with one exception: the meal is now placed on top of the table, in a space previously empty.

    Basically this is what happens when one function calls another.

    We say, in technical terms, that parameters and local variables are put in the stack. When the function returns, the stack is restaured to the way it was before.

    You need to keep in mind that everytime a function call occurs, the stat of the caller is suspended, a new space for the called function state is created, the function runs, the return value is saved, the new state disappears and the state of the caller is restored.

    There are no exceptions to this flow, at least without considering the yeld feature, but this is definitely something to be discussed else where.

    In the majority of the functions, events happen exactly like what we described here.

    But we have a problem: The world isn't exactly static, is it?

    A, my friend. The world is dynamic. If I know how many words this text here will have? No idea.
    If I am programming a game with dices and the user can choose some, I really cannot determine how many the user will choose at compile time.

    If we are receiving a JSON with some list of products in stock, we have no way of knowing how many products we have. If we did, we wouldn't possibly need systems at all.

    The problem with this schema is that I, as a programmer, have to declare exactly how much memory my function will need. And, most likely, deppending on the function, I just have no way of knowing that!

    Well, then what?

    Use the heap!

    The heap is a "general area of memory" that the operating system maintains. This is way greater than the stack, and any program can request at run-time a piece of this memory.

    The system them "separates" a piece of that huge memory area, a piece with the size the program requested, and grants access on that memory for the program requesting it.

    But, if that memory wasn't originally a part of the program's memory, how can the program access it?

    It certainly does not become part of the stack, thus the function cannot directly access it, the function can only access its stack, the area of memory allocated to it when the caller called it. This also breaks the rule that the function should specify how much memory it needs, because at run-time this is not known ahead of time.

    Then how does it work? By using pointers!

    Remember: pointers are variables which hold the address of some memory. Pointers have a fixed size. The same variable size can point to the address of a 1 byte or 150 megabytes memory.

    This means that my function still can report the exact amount of memory it needs: It can say I will need an integer, a float and a pointer. If this pointer will ever point to some address and what address it is or an address to what size it will hold is irrelevant. What counts is that, because a pointer has a fixed size, we can count its size, independently of the size of the data will be in the address it holds.

    This is how our functions, even required to have a fixed memory size, can access data of size only known at run-time.

    In fact, pointers are used widely by the operating systems to give external programs access to pieces of data they don't exactly own. This might be a view into a mapped memory, in memory addressed  by hardware devices, in files in the file system as well as memory requested at run-time by the program itself.

    Heap access doesn't come for free: because you are requesting memory at run-time, the operating system has to find, separate and isolate a block with the requested size and return an address.
    This takes some time, a long time if compared to the time it takes to allocate state for a called function on the stack.
    The memory access is also somewhat slower: a local variable in stack is accessed quickly, it is easy to calculate and populate a variable address in the stack that your function itself created at compyle time, because at compile time you know what is going on.
    In the other hand, accessing data om the heap requires first reading the address of the returned data from the pointer on stack, then accessing that address which can be far away in the memory.
    Depending on what you are writting, the difference might become noticeable.
    There are, though, two other things that need to be considered.
*/

/*
    028: Memory leaks and dangling pointers

    because the operating system gave you access to some memory, this access will be granted until you explicitely tell the operating system that you no longer need that memory.
    On stack, we know exactly when to cleanup state: when the function returns, all the memory it used is cleaned. But, at run-time, the operating system cannot understand your program specific logic.
    Neiter can it nor should it, because the operating system needs to be agnostic about what programs are running.
    What happens when a function which created a pointer returns? Well, the pointer the function created is destroied, because this pointer is om stack, and the stack is cleaned when the function returns.
    Now, we lost the address of that memory the system allocated to us in the heap. The only variable holding that address was cleaned up. We cannot reclaim that memory because we don't know the address any more. The operating system, in the other hand, will keep that memory reserved, because it has no way of knowing we are not using it.
    Congratulations, you got your first memory leak!
    Suppose you have forgoten to dispose some memory that a function called on a loop. No problems, you run your program quickly and nothing wrong happens. Then, you send it to production.
    In production, your program runs without exiting for several days, and runs the loop which call the function that allocated memory in the heap some times a day.
    After some days, memory consume on the server is high enough that it crashes. Nobody knows why, it might be a problem in the operating system, a virus, an electrical problem .. Then, what to do?
    The other way around is also a problem: Suppose you allocate some memory in the heap and pass the resulting pointer to three functions, each one responsible for doing some activity.
    You can perfectly pass a pointer as parameter to another function.
    All functions alter the data pointed by the pointer.
    After the third function returns, you read the data which should now be processed by the three functions.
    All well and good. After using the data, you let the operating system know that you no longer need that memory, so that you don't have a leak when your pointer goes away when you return.
    Then, one day, someone alters the third function your function calls. The programmer decides to optimize processing and, deppending on a if statement, request the operating system to clear that memory earlier, because it is evident that nobody will need that data anymore.
    As it is obvious, the programmer has forgoten to ask you if it would be ok to cleanup that memory inside that function. May be the programmer doesn't even know you, because we many times don't know the callers of functions we write.
    Anyways, you keep calling that function and nothing happens, because the if statement on that function, given the other parameters you pass, never becomes true.
    But, one day, you change the call. Now, the set of parameters the third function receives cause that if statement to become true. The third function then disposes the memory the pointer points to.
    But, soon after it returns, your function reads information from that same pointer. You, of course, didn't test it. Why should you at all?
    Then, in production, your program is terminated by the operating system reporting an ilegal memory access.
    Congratulations, you got your first dangling pointer.
    If you think these situations hardly happen I am extremely sorry to say you're wrong. In fact, a sigmificant part of software bugs are related to memory leaks or dangling pointers.
    You can very easily loose track of pointers when several pointers pointing to the same data are around.
    An easy way of solving both problems is by putting an intermediary component between you and the operating system, called the garbage collector.
    When you request memory, the garbage collector is aware. It gets the pointer returned by the operating system and keeps track of it.
    For you, the program, it returns another managed pointer, which it also keeps track of.
    The managed pointer contains logic to inform the garbage collector when it is going out of scope, possibly because a function holding it returned and it is being dealocated.
    Once the garbage collector realises that no managed pointers pointing to a memory area exist, it can safely dispose that memory to the operating system.
    Garbage collectors, however, can be slow, and certainly the fact that managed pointers have to report usage, does not contribute to performance.
    Programming languages have gone either the route of "you should know what you're doing, no help for you, dear programmer" or "cinse you don't know what you are doing anyways, my fellow programmer, I am obligating you to use this slow run-time, slow garbage collector anyways".
    Rust went a different way. Again, it managed to achieve the best of both worlds, by coding into the compiled binary the rules for disposing memory when appropriate, and avoid dangling pointers at all.
    How?
*/

/*
    029 - pointers, part 2

    Well, we already talked about the ownership system.
    Remember, a variable has to be owned by someone. This ownership is held by that owner untill something occurs.
    First, let's finally define that someone.
    The someone onwing variables in rust is, generally, a scope. Scopes are delimited by { and }.
    All functions have at least one scope, the function block.
    Structs have a scope, they own all fields inside it.
    After the closing brace, the scope ends. Then, ownership also ends. This is also when owned variables are dealocated, ence this is why when a function returns, all variables declared inside them will be dealocated.
    Ownership can also be moved.
    If you call a function and pass a variable you declared as parameter, you moved that variable to the called function, which now owns that variable.
    After the function returns, that variable will be dealocated.
    If you declare a variable, then declare a second variable of the same type and assign the first one to the second, you give the second variable the ownership of the value.
    You cannot use the first variable anymore. In rust, one and only one variable is the owner of a value.

    When we say you cannot use, we mean that should you try you will get a compiler error, no run-time checks need to happen.

    Types of pointers

    The most common pointer in rust is Box. This is a pointer that points to some memory in heap and has ownership of that balue.

    Notice that heap and stack are operating system concepts, ownership is a rust concept. What guarantees ownership is the rust compiler, by checking your code.

    The fact that Box retains ownership of the value tells us that no two Boxes pointing to the same memory can exist.

    If you assign a Box to another one, the first Box is not usable anymore. If you pass a Box as parameter to a function, the original function cannot use the Box anymore.
    As ownership is exclusive, whenever the scope owning the Box ends, the Box is dealocated.
    However, this is not a simple cleanup: Box has in its dealocation code special logic to inform the operating system that the memory area it points to is not needed.
    So, no memory leaks, because the Box will ask the operating system to dispose the memory, and no dangling pointers, because only one pointer can exist pointing to a give memory area.

    But Boxes are not enough. Why?

    Because there are structures which need to hold multiple pointers to the same data.

    No idea of which ones?

    Trees, for example!

    Why trees need pointers

    May be you have too much information by now. Make a pause, if so. Now, try to figure out why trees need pointers. Is it hard?

    Well, can you guess, at compile time, how many nodes a tree will have?

    Perhaps in some cases, but can a generic tree know how many nodes will be added to the root node, and then for each child how many nodes will be added to it?

    Well, hardly. This is one of the reasons we usually will need the heap to store nodes.

    "A", you say, "I was almost forgetting, but I could remember before it is too late: I can prove you know nothing about programming, and with two arguments:

    The first is that I declare rust Vectors in the stack, and vectors have no fixed size, I can push as many elements as I decide. No boxes, no pointers, we have an elastic stack, while you said the stak size for a function must be fixed."

    "And, before you so sadly try to defend yourself, I will say the second argument: in javascript, we have no pointers and, even so, I can declare arrays on stack that also grow indefinitely. I can even at ruuntime attach properties to my objects."

    Well, my dear friend, though I don't know everything about programming, I think I can explain why these two arguments seem to be valid, but they are not.

    First, a rust vector is a structure that you can allocate on stack, you're correct. But, much like our tree above, it encapsulates a pointer!
    This pointer points to some area in the heap, where elements are copied on push, and removed on pop, it so happens that there is a nice abstraction going on, so that you are using pointers, heap, drop logic and other things without knowing that. Abstractions working just in front of you!
    Second, everything in javascript you think you are creating in the stack, possibly with exception of numbers, are created in the heap.
    Arrays and objects are created in the heap, and what you get is actually a reference, a kind of managed pointer, that a garbage collector takes care of cleaning for you.
    This is why you can pass an array to a function as parameter and modify it, and you can see the changes reflected in the caller function.
    In rust, you couldn't do it because once you pass something as parameter to a function you are moving this thing to the function, thus you have no more ownership.
    In C or C++, you likely wouldn't see changes reflected, because each time you pass something as a parameter to a function data is copied (you could see it if the copy constructor did a soft copy of the backing pointer), which could lead ... to bugs.

    I hope you're satisfied, my reader, and can continue keeping up with the tree issue.

    Right, we know that, because our tree is a container and because we have no way of knowing the amount of elements it will hold, we will need the heap.
    The heap is slow, but life usually requires tradeoffs. In this case, we decided that we would need the heap.
    "But", you ask, "why didn't we use Box"?
    Remember that Box reflects the ownership model that rust uses to its values. This means that no two boxes pointing to the same memory address can exist.
    But remember, also, that tree nodes must know which node is its parent (but the root nod which has no parent), what nodes are their children nodes and which node (if applicable) comes before it and after it at the same level.
    It so happens that a Node can be the first child of another node, so it has to keep a pointer to its parent node. Four pointers pointing at the same node!
    Nop, Box wouldn't work here.
    In these situations, rust imports the notion of reference count that languages using garbage collector have.

    Rc pointers

    Rc is another kind of pointer: Differently from Box, Rc allocates a reference count together with the data it holds.
    By using a trick called deref, it allows direct access to its data, but the reference count field is there, alongside the data, and is used in two main operations:

    When you call clone(), a pointer pointing to the same structure (actual value plus reference count) is returned. The reference count increases by one.
    When the pointer is dropped (deallocated), the reference count is checked: if it is greater than one, it is decreased and the clone (only the pointer, not the area the pointer points to) is dropped.
    If the reference count is one, the operating system is called to reclaim the memory and then we have no more pointers.

    Now, I hope you are starting to see why the inner field of the structure above uses an Rc: by Using an Rc, it allows that the InnerNode, the struct which is allocated in the heap, can be referenced by multiple pointers.

    We, however, have yet another problem:

    Because Rc does not, like Box, offer guarantees of exclusivity on a given memory access, it is read only.

    This makes sense in rust terms: Using terminology from pointers, references and lifetimes (see 020), we can have a tv (the data) with several remote controls (the Rc pointers). This shouldn't give the right for all the remote control owners to try to control this tv without coordination.

    However, it is extremely hard for the rust compiler to determine if multiple write accesses are in place at compile time with Rc pointers, because heap allocated data can (and must) survive functions, specially when backed by Rc pointers.

    Because of this, there has to be a run-time control, a kind of synchronization.

    Although Rc pointers are not thread safe, the things we want to avoid here are the following:
    Function a has write access to some memory area.
    Function a starts to change that memory area state.
    Function a calls function b, sending a clone of the rc pointer.
    Function b alters the memory area.
    When function b returns, it drops the Rc pointer it received as parameter.
    But because function b had received a clone as parameter, the reference count goes to 1,  not to 0, and then the data is not dealocated.
    Function a can continue altering the memory, because its Rc pointer is still valid.
    But ... the memory area is no longer how it was before calling the function b! Function b also altered the memory area.
    This is not tolerated in rust. If it were a Box, function b would have taken ownership of the pointer, so that function a wouldn't be able to keep using that pointer.
    Rcs, as shared references, are therefore read only

    "Perfectly", you say smiling ironically. "If all reference counted pointers are read only, this means that I can have no dynamic tree, because we would have to initialize it at once. If this is so, we could very well build it on stack, and this tree is not useful at all. Switching to another more useful language!"

    Ooops, hold on. First of all, by using the heap you would be able to build a tree deppending on dynamic data, something you wouldn't be able to do on stack. But you are right. Once the tree was built, you wouldn't be able to add nodes, remove nods nor, possibly, change the elements these nods hold.
    In the other hand, making rc pointers writeable (which several other languages do without you even noticing) isn't the safest solution either. What would happen if you are iterating through the parents of nodes and sudenly you call some function that just unsets the parent node of the node you're using without you knowing?

    Refcells

    No, my friend. In rust, if you want to program in a unsafe way you will have to work really hard. The way we solve access to a given data from multiple sources is usually by providing some access control.

    In fact, this kind of "access control" is hardcoded into the compyler when you are using references, remember that the borrow checker will make sure that no two &mut references to the same data can exist, only one piece of code is able to change a given value at a time.
    But we said that it would be extremely hard for the compiler to control this kind of behavior analysing the code. Then, if the compiler can't do it, what does the language do?
    It forces you, the programmer, to take care of it!

    This is what refcells are used for: They are a kind of a gate keeper that control access to the data they own at run-time.

    If you want to read the data, you ask the gate keeper for a borrow. A nice struct with a read only reference to the data is returned, and the borrow leasts untill this struct is out of scope.
    The gate keeper can give as many read only borrows as you want.
    However, when you want to write or to alter the data, you ask for a borrow_mut. The gate keeper gives you a nice struct with a writeable reference for the data is returned, and the borrow leasts until that struct is out of scope.
    But there are rules for emiting the borrows, otherwise this would be just a useless gate keeper.
    Although you can have as many read only borrows, you can not have read only borrows active if you ask for a write borrow. Again, the rule is simmple: either one or more read borrows or one write borrow at a given time.
    Notice though that this is exactly the same thing for references: one or more & references or one &mut reference at the same time.

    "But", you ask, "when we are using references, the compiler checks this at compile time. What exactly happens if a write borrow is requested when there are active read borrows at run-time with refcells?"

    Good question. If multiple sources are trying to read and write or if multiple sources are trying to write to the same memory at the same section, we have an error. And errors are not tolerated. If you request not allowed borrows, your thread will panic, it will be terminated. If it is the main thread, your program will be terminated with an error.

    "what a strict way of thinking about life! You should not be allowed to end my program! I own my program, I decide when it ends!", you complain.

    If you want to go that route I can only ask you to reflect a little bit more: what kind of bug is worse, one which causes your program to end or one which can silently corrupt memory owned by your program and cause unknown consequences without emiting errors?

    Your program has many powers. One of them, perhaps the most dangerous one, is to cause unexpected effects to itself.

    For example, suppose you receive a static string from the command line or from the internet. Your string is stored in a struct, and an access control flag is placed in the next structure field.

    Because in memory struct fields are stored sequentially, someone can send a string one byte greater than the maximum string size. You then copy that string to the struct ... and the access control flag is changed, without you even noticing that, because the copied value covers all bytes allocated to the string plus one, and this extra byte happens to be used by your program to store the access control flag.

    "a", you say, "but this is fiction".

    I have to say you're wrong. This is a very simple form of buffer overflow, and I am sure you've heard about this kind of problem several times.

    But, may be, you are not being exploited by some bad person on internet. May be that your exploiter is ... your self or someone from your team that just performed a wrong copy. Because from the point of view of your program you're doing nothing wrong, just a copy to a piece of memory you own, this program will never report that wrong operation. It might start to give admin access to some user deppending on the size of the username for example.

    This is why the safe first approach rust offers is really exciting. In our case, we are sure that our functions are not dealing with memory in a state they are not expecting, by making sure that if the borrow is allowed, then everyone is safe to proceed.

    "but doesn't that gate keeper imposes a performance run-time?"

    Sure it does. In most part of times, though, it is perfectly acceptable. First because you're using the heap anyways. Second because if this really becomes a problem, you can always refactor your code to use less borrows.

    In last cases, you can use unsafe rust. This is perfectly fine and is the equivalent default mode of any program written in C. We won't be addressing unsafe rust here, but at this land you could create reference count data writeable by default, without checks.

    However, there is still a difference: unsafe blocks have to be marked with the unsafe keyword. This makes it easier for reviewers to concentrate efforts on analysing that part of the code and also makes it easier to know where to look for logic errors if some strange things happen in production.

    Safe rust will block most part of errors that you your self would make. If you really need performance boosts then you might first make sure a code works in safe rust and then make it unsafe and turn some checks off.

    Nothing like that is possible in C, when your whole program is at a constant risk.

    As for Rc and Refcell, even with its checking logic you are gettting a way faster operation than wat you would get with garbage collectors.

    Now, if you look at the struct where we defined our node, you will understand what is that inner field: it is a Rc pointer to a RefCell which controls access of the inner node struct.

    If you look at the impl block below, you will also be able to understand what most part of the methods do: they manage borrowing of the data inside the reffcel and manipulate that data in a way the users of this tree doesn't have to worry, bring an abstraction level.

    Wrapping up

    Ok, so mmany things covered now.

    Let's recap quickly:

    1. All functions require a fixed size for its state, defined at compile time.
    2. The way we use dynamically sized memory in a program is by requesting this memory to the system at run-time.
    3. The address of this dynamically memory is returned on a pointer, and pointers have a fixed size, so that they can be used in functions.
    4. Therefore, we have a variable of known size (the pointer) pointing to a run-time deffined memory size.
    5. Rust makes sure that when the pointer is dealocated (removed from stack when the function returns) the operating system is requested to dispose the memory area the pointer points to, so no memory leaks.
    6. Rust provides, among others, a reference counted pointer which can replicate it self while always pointing to the same memory address, allowing several functions and structs to have copies of a given pointer all pointing to the same memory address.
    7. Reference pointers only ask the operating system to dispose the memory area when the last of the replicated pointers is dealocated. No dangling pointers here.
    8. You have to use a gate keeper, called RefCell, to control read and write access to a memory pointed by an Rc pointer, because Rc pointers are by default read only. Deppending on what you do, the current thread might be terminated.
*/

/*
    030: Copying and moving

    We are not yet done, there is a second problem to discuss here.

    This is important, because it addresses a rust characteristic that is often over looked and that can put people in a situation where things just do not compile at all and there seems not to be a reason.

    Did you ever thought what happens when you pass a parameter to a function?

    Most languages, including C, perform a copy of that parameter and send this copy to the called function stack.

    Please, keep this in mind next time you are writing any program.

    Why is this so important?

    Because of some reasons:

    The first one is this: structs can be big. If function a calls function b which calls function c which calls function d all of them receiving the same struct as parameter, you have four copies of that struct happening. Put the first call on a loop ...
    The second one is that copying is not as easy: when you have only data on stack, you just make a copy of that and that's all. Although it can be an expensive operation, it is easy.
    But what about structs which have pointers to data in heap?
    If you just perform a simple copy, then you will copy the pointer, not the memory area.
    You will end with two copies of the struct, with data belonging to the struct scope replicated, and two pointers, one on each struct, pointing to the same memory area!
    Now, if you place special logic to drop this struct so that heap data is automatically dealocated when the struct is out of scope, the first copy of that struct to go out of scope will ask the operating system to dispose the heap memory.
    The second copy of that struct will have a dangling pointer.
    If you do not create logic to ask the operating system to dispose that memory, when the two structs go out of scope you will have a memory leak.
    There is another possibility: on copy, when dealing with pointers, the pointer isn't copied. Instead, another memory area is allocated on heap, and the data pointed by the pointer on the first struct is copied to the area allocated by the copied struct.
    Now, the two structs have all the data belonging to them replicated, but the pointers. These pointers are different and point to two different memory areas, which are their selfes replicated.
    But keep in mind that these heap alocated areas can also reference other heap areas, so all pointers of all structs would need to point to copied areas.
    This also presents some issues: What if some of the data are  handles for files in disk?
    Then this copy would have to copy all the files, to be considered a valid copy.
    Remember, heap access is slow. Copying memory areas of pointers of structs which are on heap pointed by other pointers of other structs that are on heap might be very expensive.
    We call the first schema, where pointers are copied, a shalow copy.
    We call the second schema, where heap memory is copied for each pointer, a deep copy.

    For someone coming from python or javascript, seeing an inocent HashMap passed as parameter in a C++ code might appear familiar. Is it really the case?

    Is this map being shalowed or depply copied? If deeply copied, how many inner structs have pointers? How is this taking place really?
    If shallowed copied, then may be the function where this map is being passed can make changes without the caller being warned.
    Sudenly, it begins to be very important to think not only in what the business logic is, but also in how the computer is performing that. Be very welcome to the lower levels.
    So that you know, copying is so important that, although c++ uses a default schema, it allows each class to define how copies should happen. For more details, search for copy constructor on any c++ documentation.
    More high level languages such as python and javascript also pass parameters as copies. Because, except for basically numbers and booleans, all data is created on heap, when you pass a "variable" as parameter, you are in reallity creating a copy of the managed reference counted pointer to that data.
    When the function returns, that copy of the pointer is destroied as the rest of that function stack is, the reference count is decreased and once nobody references that data anymore the garbage collector takes care of asking the operating system do dispose that memory.

    Rust, in the other hand, uses a completely differemt schema by default. Instead of copying parameters, as we already discussed rust moves parameters, so that they are not useable by the caller function.

    No custom behavior can be executed, the move occurs in a way the struct author cannot interfer with.

    Shallowed or deeply moved? Well, it doesn't matter. Because it is not a copy, it is a move. Remember, at any time only one variable can own a value.
    Rust doesn't even specify a moving schema, this is considered to be an internal compiler behavior, thus subject to change without further notice. The only thing you, as a programmer, needs to know is that at any moment your data can be moved, and this is not by no means under your control.

    And can you imagine what kinds of data structures are directly affected by this moving by default schema?

    Trees!

    Well, not only trees, but certainly trees are affected.

    This happens because, remember, a node has to point to its father and the father has to know its children.

    So, when we add a child node to our node, we have to give the parent field of our child the address of our selves.

    Now, remember that a struct, when passed to a function, is moved. What does it mean? Yes, it means that its address may change, if it is moved then we have no guarantees that the old address is the same, likely it is not, because a move occurs.

    So, if we pass a node from a function to another, sudenly all children of this node will have an invalid address as its father (in this case the passed node), because it has moved.

    If we pass a node differemt than the root as parameter to a function and this node has ciblings (nodes at the same level), ththe next and previous nodes will also now have invalid addresses, because they referenced the old address, and now because of the move the node likely isn't where it used to be.

    Rust is somehwat famous for making lifes of programmers who use self referencing structures (structures which needs to kknow their own addresses and use them for several things) hard. And, beliieve me, in a tree, a node is definitely a self referencing structure.

    What can we do?

    If we allocate the node in the heap, its address is fixed. It isn't part of the stack, so it is not neither copied nor moved nor dealocated unless we explicitely ask the operating system to do it.

    Remember: heap data is allocated by the operating system, has a fixed address and only is dealocated when the operating system is explicitely requested to do it.

    What is either moved or copied is (or are) the pointer (s) pointing to that address which are created on stack.

    Here, when using the Rc pointers, we either move the Rc instance (not recomended) or clone the pointer (which inbcreases the reference count) and move the clone to a function.
    The address of the data pointed by the pointer (or pointers) kkeeps the same, regardless of all the moves or copies at the stack level.

    To be complete, rust uses a copy by default schema when passing some primitive data, and can also use it for structs if the author so desires.

    The struct author can opt in to copying be default either deriving or implementing the Copy trait for its struct. Bear in mind though that copying, as we just discussed, might be more complex than what it seems to be.

    Also bear in mind that a programmer can perfectly move data from a heap area to amother, for whatever reason. This, however, is umder the programmer control, so that they have a chance to take appropriate actions when such move occurs.
    This might even happen without you noticing. For example, a Vec might move some of its data if elements in the middle are removed, if this Vec implementation guarantees that data is sequentially stored.
    Vecs, usually, are built to store elements sequentially, so that iterating through them is fast.
    This is why we don't store nodes directly in the Vec of children of a given node. We, instead, store a vec of Rc pointers, so that if elements are moved, other pointers to the heap area are still pointing to the right place (remember, a pointer being moved doesn't imply that the address it holds change).
    If yyou need to move nodes in the heap when designing a tree, you have to take care to update all pointers referencing that node to the new address.

    As a final topic in this subject, some of you might be thinking in the use of Pin to make sure data won't be moved.

    While this is a valid alternative, we won't be addressing that topic here, we don't need to over complicate things when using Rcs is enough.
*/

impl<T> Node<T> {
    pub(crate) fn create() -> Self {
        Self {
            inner: InnerNode::build_node(None),
        }
    }

    pub(crate) fn create_leaf(val: T) -> Self {
        Self {
            inner: InnerNode::build_node(Some(val)),
        }
    }

    pub(crate) fn add_leaf(&self, val: T) {
        let leaf = InnerNode::build_from_value(val);
        leaf.borrow_mut().parent = Rc::downgrade(&self.inner.clone());
        self.inner.borrow_mut().add_child(leaf)
    }

    pub(crate) fn add_node(&self, node: Node<T>) {
        self.inner.borrow_mut().add_child(node.inner);
    }

    pub(crate) fn get_first_child(&self) -> Option<Node<T>> {
        let borrow = self.inner.borrow();
        borrow.children.get(0).map(|b| Node { inner: b.clone() })
    }

    pub(crate) fn get_next(&self) -> Option<Node<T>> {
        let borrow = self.inner.borrow();
        borrow.next.upgrade().map(|p| Node { inner: p })
    }

    pub(crate) fn get_num_of_children(&self) -> usize {
        self.inner.borrow().get_num_of_children()
    }

    pub(crate) fn get_parent(&self) -> Option<Node<T>> {
        let borrow = self.inner.borrow();
        borrow.parent.upgrade().map(|p| Node { inner: p })
    }

    pub(crate) fn get_previous(&self) -> Option<Node<T>> {
        let borrow = self.inner.borrow();
        borrow.previous.upgrade().map(|p| Node { inner: p })
    }

    pub(crate) fn get_value(&self) -> NodeData<Option<T>> {
        let borrow = self.inner.borrow();
        NodeData {
            inner: cell::Ref::map(borrow, |x| &x.value),
        }
    }

    pub(crate) fn is_same_node(&self, other: &Node<T>) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }

    pub(crate) fn is_leaf(&self) -> bool {
        self.inner.borrow().is_leaf()
    }

    pub(crate) fn is_root(&self) -> bool {
        self.inner.borrow().is_root()
    }
}

pub(crate) struct NodeData<'a, T> {
    inner: cell::Ref<'a, T>,
}

impl<'a, T> Deref for NodeData<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

/*
    032: weak references

    We already discussed memory leaks and how Boxes and Rcs avoid them.

    There is, however, another form of leak we need to discuss.

    Although this is not a leak in the sense that this is not the lost address to some memory that cannot be disposed, it is a leak because it will cause the reference count to never reach 0 and, therefore, memory will never be cleaned.

    How can this happen?

    It is not that hard: you just need to create a pointer from struct a to struct b, and a pointer from struct b to struct a.

    This is a cyclic reference: if you have a pointer c to a struct a and the struct a has a pointer to struct b and the struct b has a pointer back to struct a, the reference of pointer c is two: the pointer c it self and also the pointer on struct b, both pointing at struct a.
    If you try to drop struct b, so that you can then drop the struct a, you wcan get, say, a pointer d pointing to b. Pointer d also has a reference count of two: one related to it self and another because struct a has a pointer also pointing to struct b.
    Neither reference count of pointers pointing to a nor the reference count of pointers pointing to b can reach 0, because a and b their selves have pointers pointing to each other ...
    "a, my friend!", you say, "this is why I use garbage collector languages!"
    Oh, dear reader ... I am so sorry to disapoint you: no garbage collector can detect this kind of problem. If you have been doing such things in your code in garbage collected languages, congratulations, you have created several leaks you can safely call yours. All without even realising.
    "right!", you exclaim, "if I have a parent holding a pointer to each child and each child holding a pointer to its parent, then a tree can never be dealocated. trees are not viable at all, I am going to watch tv, thanks a lot for making me loose my time!"
    Well, not kit true. In fact, there are solutions around this issue.
    If you are using C++, you can use raw pointers that have no ownership concept. You will have to remember to go and set all the pointers pointing to a node to null whenever it is removed, and checking if pointers are null before using them to access nodes.
    This would also be true for unsafe rust.
    As we decided, though, we are using safe rust and reference counted pointers to make sure we have no dangling ones around and to make sure a node is dealocated only when nobody else is using it.
    For these cases, we will introduce a new struct, called Weak.
    Weak, a short name for weak references, represents ... exactly a weak reference.
    These are pointers that can ... or perhaps may be not ... point to

    But how weak references work?
    They are similar to Rc pointers, but they don't prevent an rc pointer to dealocate its data.
    If we need to create a weak reference, we usually call downgrade() on a Rc pointer. This returns a Weak struct holding a pointer pointing to the Rc pointer.
    At the same time, a weak counter is increased on the Rc pointer. Be aware though that when a Rc is clonned, another counter, called the strong counter, is increased.
    In order to use a weak reference, you need to call the upgrade() method on it. If the data it points to is still allocated, it returns an Option enum with the Some variant containing a clone of the full Rc pointer to that data.
    If the data it points to has already been reclaimed, it returns the None variant.
    Each time a Rc is dropped, the strong reference counter is checked. The strong reference counter is, as we already discussed, increased each time a new clone is created and decreased each time this clone is dropped. Weak references are not taken in account.
    If you are asking your self how the weak reference knows the address of the data they point to, I will explain quickly that Rc pointers encapsulate their selves pointers to a struct in the heap which hold a strong counter, a weak counter and a pointer to the real data also in the heap.
    When the strong count is 0, the operating system is called to reclaim memory of the real data.
    At this time, it is not possible to clone the Rc pointer, because all instances of that Rc pointer have been dropped (we know that because the strong reference count is 0).
    If there are weak references around (the weak counter is not 0), the struct of the Rc pointer is kept, although the data this struct points to has already been reclaimed.
    In these cases, whenever you call upgrade() on any Weak clones around, None is returned back.
    Finally, when all Weak clones also go out of scope (we know that because the weak counter is 0), the struct of the Rc pointer is itself dealocated.

    So, to summarize
    1. Weak structs are created from a Rc pointer, by calling downgrade().
    2. Weak references, when created, increase on the Rc struct they refer to a weak counter. In order to do so, they keep a pointer to the Rc struct they have been created from.
    3. When you want to use a Weak struct, you call upgrade() on it.
    4. This method check the strong (not the weak) counter on the Rc struct they point to. If it is greater than 0, a normal clone of the Rc pointer the weak reference points to is returned on the Some variant of an Option enum.
    5. If the strong counter is 0, the None variant is returned.
    6. When a Rc is dropped, it decreases the strong counter on the Rc struct the Rc instance points to.
    7. If the strong counter after the decrease process is 0, the data field of the Rc struct, which is itself a pointer, is dealocated: the operating system is called to reclaim that memory.
    8. Soon after, the weak counter is checked. If it is also 0, the Rc struct itself is dealocated.
    9. But if it is greater than 0, the Rc struct is still kept allocated. On each drop of the existing Weaks, the weak counter is decreased. Once it is 0 then the Rc struct is dealocated.

    As a last comment on this topic, If you are still in doubt if garbage collectors aren't really able , to handle cyclic references, I recoment you to search for weak references for javascript and python.

    There is a main difference, though, between weak references in rust and in these other languages:
    The garbage collector is not required to dealocate objects as soon as the last strong reference to them is out of scope.
    These objects can be kept allocated for a long time before the garbage collector decides it is finally time to ask the operating system to reclaim that memory.
    If you try to use a weak reference to access an object you are sure is not in use, it might still return a valid reference, because even though no strong references are in scope anymore, perhaps the garbage collector still didn't dealocated the object.
    In the other hand, this never happens in rust, because there is no garbage collector. If a Weak, when upgraded, returns None, you can be sure that the data pointed by that Rc is no longer aallocated.
*/

struct InnerNode<T> {
    address: Weak<RefCell<InnerNode<T>>>,
    children: Vec<Rc<RefCell<InnerNode<T>>>>,
    next: Weak<RefCell<InnerNode<T>>>,
    parent: Weak<RefCell<InnerNode<T>>>,
    previous: Weak<RefCell<InnerNode<T>>>,
    value: Option<T>,
}

impl<T> InnerNode<T> {
    fn build_from_value(val: T) -> Rc<RefCell<Self>> {
        Self::build_node(Some(val))
    }

    fn build_node(value: Option<T>) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|rc| {
            RefCell::new(Self {
                address: rc.clone(),
                children: vec![],
                next: Weak::default(),
                parent: Weak::default(),
                previous: Weak::default(),
                value,
            })
        })
    }

    fn add_child(&mut self, node: Rc<RefCell<InnerNode<T>>>) {
        {
            let mut node = node.borrow_mut();
            node.parent = self.address.clone();
            if let Some(last) = self.children.last() {
                let mut last = last.borrow_mut();
                last.next = node.address.clone();
                node.previous = last.address.clone();
            }
        }
        self.children.push(node);
    }

    fn get_num_of_children(&self) -> usize {
        self.children.len()
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn is_root(&self) -> bool {
        self.parent.upgrade().is_none()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn should_create_root_node() {
        let tree = Node::<i32>::create();
        assert!(tree.is_leaf());
        assert!(tree.is_root());
    }

    #[test]
    pub fn should_create_node_element_with_value() {
        let tree = Node::create_leaf(5);
        assert!(tree.is_leaf());
        assert!(tree.is_root());
        assert_eq!(*tree.get_value(), Some(5));
    }

    #[test]
    fn should_add_leafs_to_tree() {
        let tree = Node::create();
        tree.add_leaf(1);
        tree.add_leaf(2);
        assert_eq!(tree.is_leaf(), false);
        assert_eq!(tree.get_num_of_children(), 2);
        let first_leaf = tree.get_first_child().unwrap();
        assert_eq!(first_leaf.is_leaf(), true);
        assert_eq!(*first_leaf.get_value(), Some(1));
        let second_leaf = first_leaf.get_next().unwrap();
        assert_eq!(second_leaf.is_leaf(), true);
        assert_eq!(*second_leaf.get_value(), Some(2));
    }

    #[test]
    fn should_add_node_to_tree() {
        let tree = Node::create();
        let subtree = Node::create();
        subtree.add_leaf(1);
        subtree.add_leaf(2);
        tree.add_node(subtree);
        assert_eq!(tree.is_leaf(), false);
        assert_eq!(tree.get_num_of_children(), 1);
        let first_child = tree.get_first_child();
        assert!(first_child.is_some());
        let first_child = first_child.unwrap();
        assert_eq!(first_child.is_leaf(), false);
        assert_eq!(first_child.get_num_of_children(), 2);
        let first_leaf = first_child.get_first_child().unwrap();
        assert_eq!(first_leaf.is_leaf(), true);
        assert_eq!(*first_leaf.get_value(), Some(1));
        let second_leaf = first_leaf.get_next();
        assert!(second_leaf.is_some());
        let second_leaf = second_leaf.unwrap();
        assert_eq!(second_leaf.is_leaf(), true);
        assert_eq!(*second_leaf.get_value(), Some(2));
    }

    #[test]
    fn elements_should_have_correct_relations() {
        let tree = Node::create();
        let subtree = Node::create();
        subtree.add_leaf(1);
        subtree.add_leaf(2);
        tree.add_node(subtree);
        assert!(tree.get_next().is_none());
        assert!(tree.get_parent().is_none());
        assert_eq!(tree.get_num_of_children(), 1);
        let first_child = tree.get_first_child().unwrap();
        let parent = first_child.get_parent().unwrap();
        assert!(parent.is_same_node(&tree));
        assert!(first_child.get_next().is_none());
        assert_eq!(first_child.get_num_of_children(), 2);
        let first_leaf = first_child.get_first_child().unwrap();
        let parent = first_leaf.get_parent().unwrap();
        assert!(parent.is_same_node(&first_child));
        let second_leaf = first_leaf.get_next().unwrap();
        let parent = second_leaf.get_parent().unwrap();
        assert!(parent.is_same_node(&first_child));
        let previous = second_leaf.get_previous().unwrap();
        assert!(previous.is_same_node(&first_leaf));
    }
}
