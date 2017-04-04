# The ARM toolchain prefix
ARMGNU = arm-none-eabi

# The name of the target board
BOARD = bcm2837

# The full target architecure
MARCH = armv8-a

# The target CPU
CPU = cortex-a53

# The output folder
OUTPUT = target/$(BOARD)/release/

# Directory to put the intermediate build files
BUILD = $(OUTPUT)build/

# Directory where the assembly files are
ASM = src/board/$(BOARD)/

# Name of the file to output
KERNEL = $(OUTPUT)kernel.img

# Name of the listing file to output
LIST = $(OUTPUT)kernel.list

# Name of the linker script to use
LINK_SCRIPT = src/linker.ld

OBJECTS := $(patsubst $(ASM)%.s,$(BUILD)%.o,$(wildcard $(ASM)*.s))

# Rule to build everything (creates the target image and listing)
all: $(KERNEL) $(LIST)

# Rule to create the image file
$(KERNEL): $(BUILD)kernel.elf
	$(ARMGNU)-objcopy $(BUILD)kernel.elf -O binary $(KERNEL)

# Rule to make the ELF file
$(BUILD)kernel.elf: $(OBJECTS) $(LINK_SCRIPT) xargo
	$(ARMGNU)-ld --gc-sections $(OBJECTS) $(OUTPUT)libbarelyos.a -o $(BUILD)kernel.elf -T $(LINK_SCRIPT)

# Rule to make the object files
$(BUILD)%.o: $(ASM)%.s $(BUILD)
	$(ARMGNU)-as $< -mcpu=$(CPU) -march=$(MARCH) -o $@

# Rule to make the listing file.
$(LIST): $(BUILD)kernel.elf
	$(ARMGNU)-objdump -D $(BUILD)kernel.elf > $(LIST)

# Compile the Rust code with Xargo
xargo:
	xargo build --release --target=$(BOARD)

# Create the build directory
$(BUILD):
	mkdir -p $@

# Clean all the intermediate and output files
clean:
	xargo clean
