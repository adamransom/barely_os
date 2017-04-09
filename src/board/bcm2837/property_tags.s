@;
@; Due to Rust lacking a nice way to align stack-allocated buffers, we define
@; the buffer used for property tags here.
@;

.section .data
.global __property_tags
.align 4 @; This ensures lowest 4 bits are 0 for the following label
__property_tags:
  .space 128
