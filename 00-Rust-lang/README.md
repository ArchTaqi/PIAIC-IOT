# RUST Langauge


## 4th IR

Term was introduced by KLAUS SCHWAB. Pyhical Digital and Biological 

1IR - 1760s Steam Engine
2IR - 1890s textial, industry, electricity
3IR - 1960s Digital era via Computers, Mobile and Internet
4IR - 2014s and still.... Combine speheres Digital, Pyhical and Biological.

### 4IR

- **Winner take all economy** - People with tech Knowldge, skills will grow, rest will leave behind
- **Comes in Wave** - 
    - 1st Wave 2009-2016 [Digitization, IOT, Automation, Big Data, Cloud Computing] Consumer and electronice devices will connect to IOT i.e Games, Home Devices, TVs, Cameras
    - 2nd Wave 2016-2025 [AI, Blockchain, Quantum Computing, H2M Robotics,Engergy storage] network industries devices will connect to IOT i.e Cars, Energy, Ships, Trucks, Energy
    - 3rd Wage 2025-.... [Energy storage, Neuro technology, Cyber Security, Bioinformattics, Nano tech] - Everything in the society Industry, Hospital, cities, infrastructe


- IOT Devices
    - Micro Controller - bear metal programming [rust]
- Cloud Services
- IOT Gateway [Linux, Rasbiry Pi, Kubernetes]
- Client/Mobile devices : Rust -> Web Assembly

Now Rust is native code so it does not have a virtual machine of any kind. It is a native code language. And the goal of rust is to be both powerful and safe at the same time and as such it provides an interesting alternative to languages such as C and C plus plus the go.

Cloud vs Edge Computing....

Value Proposition



https://thoughtram.io/rust-and-nickel/#/43


Resources: 
https://www.edx.org/course/getting-started-with-the-internet-of-things-iot-2
https://www.coursera.org/learn/iot 
And many other sources.



======================================================



It can replace C++ in most cases. But it cannot replace C in bare-metal embed programming.

Go is a popular language, rather simple to learn, with lot of libraries, a big community. The fact it is backed by Google helped with the latter. For programming servers, Go is an excellent choice. Very easy to pick up, you get productive in it very quickly. The built-in concurrent programming model is easier to reason about than others. The tooling is excellent, and the compiler is lightning fast. Go is a nice, simple language. Its minimalist feature set is orthogonal and powerful. Performance is not an issue; Go is faster than Java or JavaScript, the two most popular languages on the planet. And it's great for web development, too (A Word from The Beegoist)! (FYI, the Beego web framework is crazy hot in China!)


Rust is a bit harder to learn (although its syntax is relatively simple), has different concepts that must sink in your mind before becoming proficient. It has a good type system, and a powerful concept around memory management: no memory leak nor null pointer exception!

You might be more productive faster with Go, you will learn interesting things, and become a better programmer if you learn Rust... :-)

Yes, Python compiles to bytecodes (as does Java), but you still need the Python virtual machine (the VM), “hogging” memory (managing memory) when those bytecodes are run. Whether the source recompiles or not, the Python VM has to accompany the bytecode files, which slows things down.

A bare chip wouldn’t know what to do with bytecode files (unless it’s a chip customized to run Python bytecodes — these exist, see MicroPython). Java is a little different because many ARM chips have a Java bytecode mode (see Jazelle).

Rust goes straight to the chip’s assembly language, like C and C++ do. No virtual machine, no garbage collection. Rust aims to manage memory at compile time, meaning the language itself makes it hard to legally introduce memory leaks etc.

Python will likely be implemented in Rust someday soon, if it hasn’t been already.


## INTRO

## Types and Variables

## Control Flow

## Data Structure

## Functions

## Lifetime

## Odds & Ends

