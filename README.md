# libjit-sys
[libjit homepage](https://www.gnu.org/software/libjit/)

This project adds raw rust bindings around the libjit package. It is useful as a low startup time jit compiler making it ideal
for command line tools, interpreted languages, etc. Time to compile a single function that returns a number is about ~5ms on my old laptop.

# Usage
```
libjit-sys = "0.0.0"
```