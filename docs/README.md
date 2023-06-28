# Rust Glium test
if you've look through my github page you would have probably already seen the Webgl in rust project I am working on, I had comme across an error involving wasm regarding referencing a file, and parsing its data, in this case a wavefront obj file.

### Sources
http://paulbourke.net/dataformats/obj/  
https://github.com/nidirene/wfobj/  
https://github.com/glium/glium

---
# Intro
Inspired by Learn OpenGL and numerous other projects I wanted to try with rust, having implemetned the basic rendering system the next step was to work on having physics working

## why use Rust
Rust is a modern programming language that offers many advantages over C++ or C#. One of the main reasons why Rust is better than C++ or C# is its memory safety. Rust uses a system of ownership and borrowing to ensure that memory is managed correctly and efficiently, without the need for manual memory management or garbage collection. Rust also prevents common errors such as null pointers, dangling pointers, data races, and buffer overflows, which can lead to security vulnerabilities and crashes in C++ or C# programs.

## goal
the overall goal is to have an engine that has physics implemented and try working on smalelr simulations, un the fuutre being combined with other programs like ai to visualize .

---
# currently
Mesh Object containing all the data