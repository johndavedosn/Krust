# Krust
An x86_64 simple kernel made with Rust and bootimage.


# Recommendation
Compile with a nightly toolchain in order to give access for experimental features, The kernel might not compile without this.

# Building

Since the kernel uses bootimage you can install it with : 
```
$ cargo install bootimage
```
and then : 

```
$ cargo bootimage 
```
it will produce a bootable x86_64 .bin kernel in your ``target/x86_64-target/`` directory, Use something like QEMU in order to boot it.

## Booting with QEMU

Install it if you haven't already and then run : 

```
$ qemu-system-x86_64 -drive format=raw,file=path/to/kernel
```
If everything goes right you should see a QEMU window saying "Hello World!".
# Development

If you're developing on the kernel or adding any features, I have configured a bootimage runner that uses QEMU, so now you can just run :
```
$ cargo run 
```
And it will automatically compile and run the kernel using QEMU, enjoy developing!
