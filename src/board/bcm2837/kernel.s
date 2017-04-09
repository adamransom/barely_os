@;
@; This is the entry point for our kernel. All it does currently is set up the stack
@; and call the `kernel_main` function which we have defined in Rust.
@;

.section .init
.global _start @; Make _start available to the outside world

_start:
  mov sp, #0x8000 @; Set up the stack pointer
  .extern kernel_main @; `kernel_main` is defined in Rust
  blx kernel_main @; Call 'kernel_main', which never returns
  b hang @; Just in case does, hang forever

.section .text
hang:
  b hang
