# Lesson 4, Ownership, References & Borrowing

## Garbage collector (GC) 
- Garbage collection is performed by a garbage collector which recycles memory that it can prove will never be used again. Garbage collection is typically used periodically or on demand, like if the heap is close to full or above some threshold. It then looks for unused variables and frees their memory, depending on the algorithm.
- In Rust, when the variable gets out of scope or its lifetime ends at compile time and a `drop` function is called to to free the memory.                                                                                                      
More on [Rust has a static garbage collector](https://words.steveklabnik.com/borrow-checking-escape-analysis-and-the-generational-hypothesis), [stackoverflow](https://stackoverflow.com/questions/32677420/what-does-rust-have-instead-of-a-garbage-collector)

## Stack
The stack and heap are ways of managing memory at runtime.

- store primitives data types / static allocated types ex: int
- fixed size
- fast because it stores and accesses data based on order, space needed to stack is known at compile time.


## Heap

- store compound data types / dynamic allocated types ex: array
- dynamic in size/unknown-size needs to be stored during runtime
- slow

> For any statically allocated types, Rust Ownership and Borrowing doesn't apply but for dynamic/heap allocation it does apply.

**Shallow vs Deep Copy**

A **deep copy** occurs when an object is copied along with the objects to which it refers. **Shallow copy** is a bit-wise copy of an object. A new object is created that has an exact copy of the values in the original object.

```rust
let s1 = String:from('Hello');
let s2 = s1;     // shallow copy, copies the content of stack only

let s3 = s1.clone();   // Deep copy, move the stack and heap to new memory hold by s3
```

> Keep in mind that calls to clone() can be expensive, which is why Rust prevents this “deep copying” by default.


## Ownership & Scope

Rust neither has manual memory management nor garbage collector. Then the question would be, how does Rust manage memory and the answer is Ownership.

**Ownership Rules**
- Each value in Rust has a variable that’s called its `owner`.
- There can only be one owner at a time.
- When the owner goes out of scope `{}`, the value will be dropped.

```rust
fn main(){
    let mut name = String::from('Muhammad Taqi');
    name.push_str(' Hassaan');
} // scope ends here and memory taken by `name` is freed/deallocated.
```

**Double Free Error**

Double free error occurs when try to freed memory twice. This can happen when two or more var point to same address in memory.

```rust
let a = 10;
let b = &a;
// when scope ends it tries to free memory by a first, but it's still pointed by b.
```
Rust has a solution for this; when scope ends a give up the ownership of address of a and b now holds the ownership. So when when scope ends `drop` function will freed memory hold by b.


## Ownership in Functions

- when we pass a stack data type to a function, it passes a copy of that variable. The original variable data and its ownership will still be available to you in the function where it is passed.
```rust
fn main() {
    let number: i32 = 1;
    let result = do_something(number);
    println!("result is {} for number {}", result, number)
}

fn do_something(input: i32) -> i32 {
    input * 2
}
```
Output: result is 2 for number 1

- When we pass a heap data type to a function, it moves the variable and it's ownership to that function and no longer will be available in the function from where it is passed.

```rust
fn main() {
    let name = String::from("Hello, ");
    do_something(name);
    println!("name is {}", name);
}

fn do_something(name: String) {
    println!("name is {}", name);
}

#output: error
do_something(name);                 ---- value moved here
println!("name is {}", name);       ---- value borrowed here after move
```

```rust
fn main() {
    let name = String::from("Hello, ");
    let (_len, _val) = check_len(name);
    println!("name is {}", name);
}

fn check_len(name: String)->(usize,String) {
    (name.len(),name)
}
#output:error
println!("name is {}", name);   ---- value borrowed here after move
```

## Borrowing
- Ref -> &
- DeRef -> *

```rust
let a = 10;
let b = &a;
// a's adress has value 10
// b is a pointer and it's adress refers to adress of a.
```

```rust
fn main() {
    let mut name = String::from("Hello, ");
    let _len = check_len(&mut name);
    println!("name is {}", name);
}

fn check_len(name: &mut String)->(usize) {
    name.push_str("World!");
    name.len()
}
```
Now we have only pass address/ref of `name` variable to `check_len` function, ownership is still in the main function and its not destroyed yet.


## Mutable and Immutable Reference

- Mutable -> read & write
- Immutable -> Read only

**Data Race** When two or more variables point to an `mut var` in the same scope then race condition occurs. Usually happens when;
- Two or more pointers access the same data at same time
- At least one of the pointer is used to write data
- There is no mechanism used to syncronize data.

```rust

```

**Solution**
```rust
```


## Dangling Reference



## String literal

- string literal `&str` stored on stack
- String stored on Heap.


## More Resources

1. https://medium.com/@thomascountz/ownership-in-rust-part-1-112036b1126b

