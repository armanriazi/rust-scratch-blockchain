### Currently Status: Under Refactoring
> The repo is included Rust syntax, configuration and the goal of creating scratch codes like one is becuase of having testbed environment of Blockchain.
 
> The Next reason is to using some features of Rust-Lang that I had wanted to implement it after learning Rust.
### Used Concepts

#### Memoization•Lazy•Evaluation(Passed)  
   > We can create a struct that will hold the closure and the resulting value of calling the closure.
  
> The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result.
> 
> FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined.
> The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
> 
> FnMut can change the environment because it mutably borrows values.
> 
> Fn borrows values from the environment immutably. FnOnce: takes the whole value. FnMut: takes a mutable reference. Fn: takes a regular reference.

#### Coercion(Passed)
  > Deref coercion is a convenience that Rust performs on arguments to functions and methods. Deref coercion works only on types that implement the Deref trait. Deref coercion converts such a type into a reference to another type. For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str. 
> The number of times that Deref::deref needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!
Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.the Drop trait is almost always used when implementing a smart pointer. For example, when a Box<T> is dropped it will deallocate the space on the heap that the box points to.
> Note that we didn’t need to call the drop method explicitly.
  
#### DST•Or•Unsizedtype(Passed)
  > DSTs or unsized types': str(but not &str-So although a &T is a single value that stores the memory address of where the T is located, a &str is two values: the address of the str and its length. Rust has a particular trait called the Sized trait to determine whether or not a type’s size is known at compile time. This trait is automatically implemented for everything whose size is known at compile time. In addition, Rust implicitly adds a bound on Sized to every generic function. 

#### Operation(Passed)
  -> Methods are functions that are coupled to some object.
  From a syntactic point of view, these are just functions that don’t need to specify one of their arguments. Rather than calling open() and passing a File object in as an argument (read(f, buffer)), methods allow the main object to be implicit in the function call (f.read(buffer)) using the dot operator.
	
> There are a number of theoretical differences between methods and functions, but a detailed discussion of those computer science topics is available in other books. Briefly, functions are regarded as pure, meaning their behavior is determined solely by their arguments. Methods are inherently impure, given that one of their arguments is effectively a side effect. These are muddy waters, though. Functions are perfectly capable of acting on side effects themselves. Moreover, methods are implemented with functions. And, to add an exception to an exception, objects sometimes implement static methods, which do not include implicit arguments.
To define methods, Rust programmers use an impl block
  
#### Borrowchecker(Passed)
  > The borrow checker checks that all access to data is legal, which allows Rust to prevent safety issues. Learning how this works will, at the very least, speed up your development time by helping you avoid run-ins with the compiler. More significantly though, learning to work with the borrow checker allows you to build larger software systems with confidence. It underpins the term fearless concurrency.
  
#### Borrowchecker•Lifetime(Passed)
  -> Lifetime=Timetolive=Subset of their scope
	
  Make hypotheses about whether or not your experiments will pass the borrow checker before you compile
> reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
	
> The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.
All references in Rust have a lifetime, even if they are not explicitly annotated. The compiler is capable of implicitly assigning lifetimes. 
> A value’s lifetime is the period when accessing that value is valid behavior. A function’s local variables live until the function returns, while global variables might live for the life of the program.
	
> The notion of ownership is rather limited. An owner cleans up when its values’ lifetimes end.
	
> <'a, 'b> declares two lifetime variables, 'a and 'b, within the scope of
	
> j: &'b i32 binds the lifetime variable 'b to the lifetime of j. The syntax reads as “parameter j is a reference to an i32 with lifetime b.”
	
> Although every parameter has a lifetime, these checks are typically invisible as the compiler can infer most lifetimes by itself.
	
> All values bound to a given lifetime must live as long as the last access to any value bound to that lifetime.
	
> No lifetime annotations are required when calling a function.
	
> Using two lifetime parameters (a and b) indicates that the lifetimes of i and j are decoupled.
> fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {}
	
> Lifetime annotations don’t change how long any of the references live. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.
	
> Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
	
> The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.
	
> Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
	
> Although every parameter has a lifetime, these checks are typically invisible as the compiler can infer most lifetimes by itself
	
> All values bound to a given lifetime must live as long as the last access to any value bound to that lifetime.
	
> No lifetime annotations are required when calling a function.
	
> Using two lifetime parameters (a and b) indicates that the lifetimes of i and j are decoupled.
	
> fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {}
	
> lifetime of that usage: 
	
>> the LOC('existence time' or Line of code) between when a location is first used in a certain way, and when that usage stops.
	
>> lifetime of that value:
	
>> the LOC (or actual time) between when a value is created, and when that value is dropped.
	
> Might be useful when discussing open file descriptors, but also irrelevant here.
	
> Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

     
#### Dangle(Passed)  
  > The main aim of lifetimes is to prevent dangling references.which has an outer scope and an inner scope.
In return section of a function primitive types need to define as (&'a or &'static)

#### Generic(Passed) 
  > You might be wondering whether there is a runtime cost when using generic type parameters. The good news is that using generic types won't make your run any slower than it would with concrete types.
Rust accomplishes this by performing monomorphization of the code using generics at compile time. 
Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compile.
Every programming language has tools for effectively handling the duplication of concepts.
In Rust, one such tool is generics. Generics are abstract stand-ins for concrete types or other properties. When we’re writing code,
we can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.

#### Static•Dispatch(Passed)
    -> Monomorphization
   > Dispatch is the mechanism to determine which specific version of code is actually run when it involves polymorphism. Two major forms of dispatch are static dispatch and dynamic dispatch. While Rust favors static dispatch, it also supports dynamic dispatch through a mechanism called ‘trait objects’.
When Rust compiles this code, it performs monomorphization.
> The monomorphized version of the code looks like the following. The generic Option<T> is replaced with the specific definitions created by the compiler:
versions of a polymorphic function (or any polymorphic entity) during compilation is called Monomorphization.
> Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. When the code runs, it performs just as it would if we had duplicated each definition by hand. The process of monomorphization makes Rust’s generics extremely efficient at runtime.
This is opposed to dynamic dispatch
  
#### Dynamic•Dispatch(Passed)
  > The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.
This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling.
In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
> When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that is using trait objects,
so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.

#### Blanket•Implementation(Passed)
  > Any implementation where a type appears uncovered. impl<T> Foo for T, impl<T> Bar<T> for T, impl<T> Bar<Vec<T>> for T, and impl<T> Bar<T> for Vec<T> are considered blanket impls.
> We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait.

#### Bound(Passed)
  > Bounds are constraints on a type or trait. For example, if a bound is placed on the argument a function takes, types passed to that function must abide by that constraint.

#### Trait(Passed)
  > We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.
Traits are similar to a feature often called interfaces in other languages, although with some differences.
What is a trait? A trait is a language feature that is analogous to an interface, protocol, or contract. If you have a background in object-oriented programming, consider a trait to be an abstract base class. If you have a background in functional programming, Rust’s traits are close to Haskell’s type classes
these also support a form of inheritance that’s common in most object oriented languages. For now, though, the thing to remember is that traits represent common behavior (Or reusable codes like println!)that types opt into via the syntax impl Trait for Type.
> After the method signature, instead of providing an implementation within curly brackets, we use a semicolon.
> This interface consists of associated items, which come in three varieties:
> 	functions
> 	types
> 	constants
> All traits define an implicit type parameter Self that refers to "the type that is implementing this interface".
> Trait functions may omit the function body by replacing it with a semicolon. This indicates that the implementation must define the function. If the trait function defines a body, this definition acts as a default for any implementation which does not override it. Similarly, associated constants may omit the equals sign and expression to indicate implementations must define the constant value. Associated types must never define the type, the type may only be specified in an implementation.

  
#### Polymorphism(Passed)
  > In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated, whereas in other languages, the data and behavior combined into one concept is often labeled an object.However, trait objects are more like objects in other languages in the sense that they combine data and behavior.

#### Unrolling(Passed)  
  > It is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.

#### Binding•Match(Passed)
  > The compiler automatically references the Some, and since we're borrowing, name is bound as ref name automatically as well. If we were mutating:
  ```
   //https://blog.rust-lang.org/2018/05/10/Rust-1.26.html#nicer-match-bindings
  // `self` has type `&List`, and `*self` has type `List`, matching on a
  // concrete type `T` is preferred over a match on a reference `&T`
  // after Rust 2018 you can use self here and tail (with no ref) below as well,
  // rust will infer &s and ref tail. 
  ```
	
#### Datarace•Rustaceans(Passed)
  > Note: The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

#### Nan(Passed)
  > Floating-point types include “not a number” values (represented in Rust syntax as NAN values) to handle these cases.
> NAN values poison other numbers. Almost all operations interacting with NAN return NAN. Another thing to be mindful of is that, by definition, NAN values are never equal. 
> To program defensively, make use of the is_nan() and is_finite() methods. Inducing a crash, rather than silently proceeding with a mathematical error, allows you to debug close to what has caused the problem. The following illustrates using the is_finite()

#### Duplication((literal)  
  > Concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy.
If we do want to deeply copy the heap data of the String, not just the stack data, we can use a common method called clone

#### Semantic(literal)
 > Primitive types are said to possess copy semantics, whereas all other types have move semantics.
Adding more functionality (e.g., reference-counting semantics rather than move semantics) to types by wrapping these in other types typically reduces their run-time performance.
  
#### Zero•Cost•Abstractions(literal)
 > One of the ways this manifests is by not adding extra data around values within structs.
  
#### Coherence(literal)
	-> Orphan = Trait•External•Implement
  > But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are defined in the standard library and aren’t local to our aggregator crate.
This restriction is part of a property of programs called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. 
> Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.
> Preserves contextual coherence of trace data from tasks/function/methods when logging. 
> For example new instance of a struct of course, as you probably already know, struct then you can just summerize your struct in a method.

#### Jargon(literal)
  > Functional programming jargon: “to cons x onto y” informally means to construct a new container instance by putting the element x at the start of this new container, followed by the container y.Other, more complex recursive data types are useful in various situations, but by starting with the cons list, we can explore how boxes let us define a recursive data type without much distraction.

#### Refactor(literal)
  > One alternative to refactoring is to simply copy values. Doing this often is typically frowned upon, however, but it can be useful in a pinch. Primitive types like integers are a good example of that. Primitive types are cheap for a CPU to duplicate—so cheap, in fact, that Rust always copies these if it would otherwise worry about ownership being moved.
> Types can opt into two modes of duplication: cloning and copying.
  
#### Pattern•Newtype
  > Using the Newtype Pattern to Implement External Traits on External Types 'thin wrapper around the type' : part of Vec<String> is noticed. We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T> value The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding. We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>.
> If we wanted the new type to have every method the inner type has, implementing the Deref trait (If we don’t want the Wrapper type to have all the methods of the inner type—for example, to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.)

#### Pattern•Design•Interior(future work)
   > Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing. 
> RefCell<T> type that follows the interior mutability pattern.
> Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds. So, what makes RefCell<T> different from a type like Box<T>? Recall the borrowing rules...
> Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.
At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.References must always be valid.

#### Type•Wraper(future work)  
	-> Wrapper type = Reference-Counted Value = Shared Ownership = Track valid references
  > Use wrapper types, which allow more flexibility than what is available by default. These, however, incur costs at runtime to ensure that Rust’s safety guarantees are maintained. Another way to phrase this is that Rust allows programmers to opt in to garbage collection.
  
#### Mem•Leak(future work)
	-> Managing Memory Leak
  > Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak). Preventing memory leaks entirely is not one of Rust’s guarantees in the same way that disallowing data races at compile time is, meaning memory leaks are memory safe in Rust. We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle. This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

##### Mem•Doublefree(future work)
 This is a problem: when s2 and s1 (s2 is copied s1 means 2different pointer and the same data) go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs we mentioned previously. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
 
#### Mem•Deallocating•or•RAII(future work)
 Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.
  
#### Thread(future work)
 Which parts of your code on different threads will run. This can lead to problems, such as:
Race conditions, where threads are accessing data or resources in an inconsistent order Deadlocks, where two threads are waiting for each other to finish using a resource the other thread has, preventing both threads from continuing Bugs that happen only in certain situations and are hard to reproduce and fix reliably.
  
#### Thread•Strateges(future work)
  -> Priority Performance
> Stealing_Join
execute code in parallel when there are idle CPUs to handle it.
When join is called from outside the thread pool, the calling thread will block while the closures execute in the pool. When join is called within the pool, the calling thread still actively participates in the thread pool. It will begin by executing closure A (on the current thread). While it is doing that, it will advertise closure B as being available for other threads to execute. Once closure A has completed, the current thread will try to execute closure B; if however closure B has been stolen, then it will look for other work while waiting for the thief to fully execute closure B. (This is the typical work-stealing strategy).
Send is require because we have jump from quick func(thread a) to part func(thread b) frequently.
> Atomic
Atomic types provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types.
This module defines atomic versions of a select number of primitive types, including AtomicBool, AtomicIsize, AtomicUsize, AtomicI8, AtomicU16, etc. Atomic types present operations that, when used correctly, synchronize updates between threads.
Each method takes an Ordering which represents the strength of the memory barrier for that operation. These orderings are the same as the C++20 atomic orderings. For more information see the nomicon.
Atomic variables are safe to share between threads (they implement Sync) but they do not themselves provide the mechanism for sharing and follow the threading model of Rust. The most common way to share an atomic variable is to put it into an Arc (an atomically-reference-counted shared pointer).
Atomic types may be stored in static variables, initialized using the constant initializers like AtomicBool::new. Atomic statics are often used for lazy global initialization.
> Spin_Loop_Yeild
  also known as busy loop and spin loop-If you want to sleep pause a thread for short amounts of time, or if your application is sensitive to timing, use a spin loop


#### Unsafe•Extern•Mangling(future work)
  Mangling is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.

#### Superpower(future work)
if the Rust compiler doesn’t have enough information to be confident, it will reject the code. In these cases,
you can use unsafe code to tell the compiler, “Trust me, I know what I’m doing.” The downside is that you use it at your own risk: 
if you use unsafe code incorrectly, problems due to memory unsafety, such as null pointer dereferencing, can occur.
You can take five actions in unsafe Rust, called unsafe superpowers, that you can’t in safe Rust. Those superpowers include the ability to:
  Dereference a raw pointer
  Call an unsafe function or method
  Access or modify a mutable static variable
  Implement an unsafe trait
  Access fields of unions
  Calling unsafe() would crash the program.
consider unsafe to be a warning sign rather than an indicator that you’re embarking on anything illegal. Unsafe means “the same level of safety offered by C at all times.” 
If you still had access to (via unsafe) they might still look like valid S, but any attempt to use them as valid S is undefined behavior. ↓
https://cheats.rs/#unsafe-unsound-undefined-dark side of force
Try to avoid "unsafe {}", often safer, faster solution without it. Exception: FFI.
People are fallible, and mistakes will happen, but by requiring these five unsafe operations to be inside blocks annotated with unsafe you’ll know that any errors related to memory safety must be within an unsafe block. Keep unsafe blocks small; you’ll be thankful later when you investigate memory bugs.
To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API, which we’ll discuss later in the chapter when we examine unsafe functions and methods.
Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited.
Wrapping unsafe code in a safe abstraction prevents uses of unsafe from leaking out into all the places that you or your users might want to use the functionality implemented with unsafe code, because using a safe abstraction is safe.

  
#### OOP•State•DesignPattern(future work)
 -> We can used it for smart contracts so we will need to implemented smart contracts
 Using the state pattern means when the business requirements of the program change, we won’t need to change the code of the value holding the state or the code that uses the value. We’ll only need to update the code inside one of the state objects to change its rules or perhaps add more state objects.
e.g Post type. This type will use the state pattern and will hold a value that will be one of three state objects representing the various states a post can be in—draft, waiting for review, or published. Changing from one state to another will be managed internally within the Post type. The states change in response to the methods called by our library’s users on the Post instance, but they don’t have to manage the state changes directly. Also, users can’t make a mistake with the states, like publishing a post before it’s reviewed.

  ### How To Contribute(Easy)
[HowToContribute](https://github.com/armanriazi/armanriazi/blob/main/HowToContribute.md)
