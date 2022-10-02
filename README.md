# Not ready to be used, currently just a prototype with no stability guarantees.
Even though the interface has mostly stabilised I still keep finding bugs in its output and have a habit of rewriting history. Feedback or contribution is welcome, however please keep in mind that I have absolutely no experience with public projects.

# pumice
Even less safe Vulkan bindings for Rust.

## Why
There are already many bindings for Vulkan and so in the spirit of reinvention here you have another. That said, there are notable differences:
* Generating only a subset of the Registry, this is likely the main feature and allows generating bindings only for selected extensions and core versions.
* An attempt at making the generated code readable as-is, for example all enum variants are grouped together rather than split across multiple files.
* No struct initialization with the builder pattern, instead just initialize the structs directly (though the Default implementations are aware of things like `sType`). This loses the option of using lifetimes for the contained pointers and slices must manually be decomposed into a pointer and length, I consider this to be more readable and it should improve compile times and debug performance.
* A best-effort attempt at generating `Hash` and `Clone`-like implementations for structs. This does consider objects referred to by pointers and copy/hash them, however occasionally the field is a void pointer which provides no information about its contents, in these cases it is not copied and only the adress is hashed. Float identity is their bit pattern and the generated code is a safety nightmare so you should probably reconsider using this.
* `pNext` chain traversal - a macro that templates user code into a match statement which handles the traversal.
* Offensively unsafe choices like potentially dangling pointers in the public interface, bacause segfaults are great fun.
* Both raw ffi functions corresponding to function pointers loaded from the driver and high-er level ones that are easier to use (essentially stolen from Erupt).
* Fully generated Vulkan Video extensions support.

## Other projects
* [erupt](https://gitlab.com/Friz64/erupt) - The main inspiration for this project and the one you should probably use, every part is generated so it should be tho most up to date and consistent.
* [vulkano](https://github.com/vulkano-rs/vulkano) - High level handwritten bindings that can automatically manage synchronization. 
* [ash](https://github.com/ash-rs/ash) - The most widely used bindings with the largest ecosystem, though the generator is truly disgusting and efforts to rewrite it or switch to some other are slow going.
* [vulkanalia](https://github.com/KyleMayes/vulkanalia/blob/master/README.md) - Includes a full port of the Vulkan Tutorial.
* [vulkan-bindings](https://github.com/bsekura/vulkan-bindings) - Seemingly the most barebones low level bindings.
