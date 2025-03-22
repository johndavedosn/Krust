# Krust
An x86_64 simple kernel made with Rust and bootimage.

#  Discontinuation notice.
As Krust is currently awfully outdated and actually broken,   I have decided to leave the project 
and leave it as a learning codebase more so than an actually functioning kernel.
## Updated.
I have fixed a lot of issues within the kernel and managed to upgrade ``x86_x64`` to the newest version (it's the most important crate in my eyes) fixing all the issues that came with,   though it seems there is an issue still with the target that needs fixing so yeah....still can't compile.
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
