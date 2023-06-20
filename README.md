# Big rewrite is underway, though I'm currently taking a break from being angry at the vk.xml format.

## Why
For some reason everyone keeps writing Vulkan bindings generators, so I thought I'd join the fun. That said, there are some differences to the popular alternatives:
* Enabling only select extensions / core versions. This was previously implemented in the generator, a rewrite to use Rust features is being worked on.
* Extensions are all available from a single table (or globally, if enabled) rather than each having an object that needs to be initialized and passed around. Using a symbol from an unloaded extension leads to a panic.
* No struct initialization with the builder pattern, instead just initialize the structs directly (though the Default implementations are aware of things like `sType`). This loses the option of using lifetimes for the contained pointers, slices must manually be decomposed into a pointer and a u32 length, I consider this to be more readable and it should improve compile times and debug performance.
* Fully generated Vulkan Video extensions support.

## Other projects
* [erupt](https://gitlab.com/Friz64/erupt) - The main inspiration for this project and the one you should probably use, every part is generated so it should be tho most up to date and consistent.
* [vulkano](https://github.com/vulkano-rs/vulkano) - High level handwritten bindings that can automatically manage synchronization. 
* [ash](https://github.com/ash-rs/ash) - The most widely used bindings with the largest ecosystem, though the generator is truly disgusting and efforts to rewrite it or switch to some other are slow going.
* [vulkanalia](https://github.com/KyleMayes/vulkanalia) - Includes a full port of the Vulkan Tutorial.
* [vulkan-bindings](https://github.com/bsekura/vulkan-bindings) - Seemingly the most barebones low level bindings.
