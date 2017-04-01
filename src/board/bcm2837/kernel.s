@;
@; This turns on the ACT LED for the Raspberry Pi 3 Model B v1.2
@;
@; The ACT LED is no longer wired directly to a GPIO pin and now belongs on
@; the GPIO expander, which is controlled by the GPU. In order to communicate
@; with the GPIO expander, we need to use the GPU's mailbox interface (in
@; particular, we need to send a message to the property tag channel).
@;
@; Mailbox base address: 0x3f00b880
@; Mailbox 1 write address: [0x3f00b880, #0x20]
@; Property tag channel: 8
@; Property tag ID: 0x00038041 (SET_GPIO_STATE)
@; Property tag message: 130 1 (ACT_LED pin number followed by state)
@;

.section .init
.global _start @; Make _start available to the outside world

_start:
  mov sp, #0x8000 @; Set up the stack pointer
  .extern kernel_main @; Declare an external symbol
  blx kernel_main @; Call the 'kernel_main' function defined in Rust
  b main @; Run main, which never returns

.section .text
main:
  ldr r0, =1000000 @; 1 second
  bl Wait @; Wait a bit

  bl LED_ON

  ldr r0, =1000000 @; 1 second
  bl Wait @; Wait a bit

  bl LED_OFF

  b main
