# Barely OS

The goal of this project is to eventually write an extremely simple kernel for the Raspberry Pi 3, to learn more about bare-metal programming and how operating systems work in general.

At first, this will mostly be experiments with bare-metal programming in assembly, then transitioning into Rust and finally starting to work towards building a kernal.

I hope to record at least some of this journey on my blog over at [adamransom.github.io](https://adamransom.github.io), though I can't vouch that'll be updated all that frequently.

## Setup

In order to run these examples, you will most notably need a Raspberry Pi 3 Model B. Whilst a lot of the same code can be used on Raspberry Pi 2, there are some big differences early on (for example when trying to turn on the ACT LED).

Secondly you will need the [arm-none-eabi](https://launchpad.net/gcc-arm-embedded/+download) toolchain to assemble, link and generate a binary image to put on your SD card.

Finally you will need [make](https://www.gnu.org/software/make/) if you want to make building the examples easier.

## Usage

If you have `make` installed, all you need to do is run:

```Bash
make
```

Then you will need to copy the generated `kernel.img` to your SD card, along with the [Raspberry Pi boot files](https://github.com/raspberrypi/firmware/tree/master/boot) (`bootloader.bin` and `start.elf`), put the card carefully in your Pi and you are good to go!

## What it does

Well, right now the small assembly file simply turns on the activity LED on the Raspberry Pi 3. Hopefully a lot more interesting things will happen soon!
