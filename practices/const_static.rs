// Stack
// Temporary.
// Variables go in and out of existence as functions are called/returned.
// Freed automatically when scope ends.

// Heap
// Dynamically allocated while program runs.
// Needs explicit owner to free before program ends (otherwise → memory leak if the program runs long).

// Static / Read-only memory
// Baked into the binary when the program is compiled.
// Data is available for the entire lifetime of the program.
// You don’t allocate or free it manually — it just exists as long as the program is loaded.

//🔹  const
// The compiler inlines the value.
// That means whenever you use const NAME, the compiler replaces it with the literal value directly in the code.
// There is no memory address for a const. It doesn’t exist as a variable in memory at runtime.
//         Example
//         const N: i32 = 42;
//         fn main() {
//             let a = N;
//             let b = N + 1;
//         }
//         👉 In assembly, a = N compiles to something like:
//             mov eax, 42
//             So the number 42 is inside the instruction stream (in .text) as an immediate constant.
//             No memory slot for N exists.

//             If you write a large array:
//             const TABLE: [u8; 3] = [1, 2, 3];
//             The compiler can’t encode the whole thing as immediate operands, so it puts [1,2,3] into .rodata, and instructions will reference it by address.
//                 ✅ So:
//                     Small values → embedded as immediates in .text.
//                     Large values → stored in .rodata.
//                     Never .data.
//                     Never .bss.

// 1. Instructions vs Data
// When you see:
    mov eax, 42
    // The 42 is encoded inside the instruction bytes in .text.
    // Example encoding: B8 2A 00 00 00 (where B8 means move immediate into eax, and 2A 00 00 00 is the literal 42).
    // Those bytes are at some address — yes — but that address is the address of the instruction, not the address of a separate value in memory.
    // So 42 is part of the machine code itself. It has no independent symbol or slot.

// 2. With static
        // static N: i32 = 42;

        fn main() {
            let a = N;
        }
        // The compiler puts 42 in .rodata as data bytes.
        // Example: .rodata contains 2A 00 00 00 at address 0x600010.
        // Then the instruction looks like:
        // mov eax, DWORD PTR [0x600010]   ; load from memory at address 0x600010
        // Here, N has its own independent address (0x600010).


// Difference:
// const: value lives inside instruction bytes in .text. The only “address” is the address of the instruction itself. There’s no separate symbol.
// static: value lives in its own slot in .rodata/.data. It has a dedicated address and symbol.

// where is symbol used:
//     🔹 How instructions use symbol
//         In the assembly language the compiler writes symbol for simplicity before the linker assigns final addresses, 
//         you won’t usually see 0x600010 directly.
//         You’ll see:
//             mov eax, DWORD PTR [rip + offset_to_N]   ; load value of symbol N
//         Here:
//             N is the symbol.
//             The assembler/linker later replaces it with the actual numeric address (0x600010).


// When you compile a Rust program, the compiler produces a binary file (an executable).
// That binary is divided into sections. The most important ones:

// .text → the actual machine code (functions).
// .rodata → read-only data (constants, immutable statics, string literals). read-only memory of the program
// .data → writable global data (for static mut).
// .bss → space for uninitialized global variables.

//.text:
// The .text section of the program binary is where all the machine code instructions live.
// Each instruction in this file is written in memory one after another.
// Since .text is byte-addressable, every single instruction has a numeric address (like 0x401000, 0x401004, etc.).
// When your program is loaded, the OS copies the machine code from the executable file into RAM.
// That part of RAM is marked as read-only + executable (so the CPU can run it but you can’t modify it like normal data).

// 🏗 Where does a function live?
// When you compile a Rust (or C, or C++) program, all the machine code for your functions is placed in a special part
// of the binary called the text section (also called code segment).
// functions are just instructions baked into the binary file(.text).

// 📍 What is an "address" in ?
// Every instruction in program memory has a numeric address (like a street address).
// Think of your program’s memory as a long street:

// 0x401000:   <machine code for add_one starts here>
// 0x401004:   <next instruction>
// 0x401008:   ...
// 0x401050:   <machine code for add_two starts here>

// Here:
//     add_one “lives” at 0x401000
//     add_two “lives” at 0x401050
//     That number is the address.

// 🔹 Case 1: Function item (direct function)
//     fn add_one(x: i32) -> i32 { x + 1 }

//     let y = add_one(5);

//     Here:
//         The compiler knows at compile time: "add_one lives at 0x401000".
//         So it doesn’t even bother with variables.
//         It just burns the address directly into the machine instruction:

//         call 0x401000    ; hardcoded address

//         No indirection. No lookup.
//         That’s why we say a function item is zero-sized — there’s no data to store, only code.

// 🔹 Case 2: Function pointer (variable holding an address)
//     fn add_one(x: i32) -> i32 { x + 1 }
//     fn add_two(x: i32) -> i32 { x + 2 }

//     let mut f: fn(i32) -> i32 = add_one;
//     f = add_two;
//     let y = f(5);

//     Here:
//         The variable f actually lives in memory (on the stack or register).
//             At runtime:
//                 First, f stores the number 0x401000 (add_one).
//                 Later, f stores 0x401050 (add_two).
//                 When calling, the CPU must first read f to see which address is stored:

//                 mov rax, [f]   ; load the address from the variable f
//                 call rax       ; indirect jump to wherever f points

//                 That’s why we call it an indirect call.
//                 The CPU doesn’t know at compile time which function f will point to.
