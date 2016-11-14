# The ARM toolchain prefix
ARMGNU = arm-none-eabi

# The target architecture
ARCH = armv8-a

# The target CPU
CPU = cortex-a53

# The output folder
OUTPUT = target/$(CPU)/release/

# Directory to put the intermediate build files
BUILD = $(OUTPUT)build/

# Directory where the source files are
SOURCE = src/

# Name of the target
TARGET_NAME = kernel

# Name of the file to output
TARGET = $(OUTPUT)kernel.img

# Name of the listing file to output
LIST = $(OUTPUT)kernel.list

# Name of the linker script to use
LINK_SCRIPT = kernel.ld

OBJECTS := $(patsubst $(SOURCE)%.s,$(BUILD)%.o,$(wildcard $(SOURCE)*.s))

# Rule to build everything (creates the target image and listing)
all: $(TARGET) $(LIST)

# Rule to create the image file
$(TARGET): $(BUILD)kernel.elf
	$(ARMGNU)-objcopy $(BUILD)kernel.elf -O binary $(TARGET) 

# Rule to make the ELF file
$(BUILD)kernel.elf: $(OBJECTS) $(LINK_SCRIPT) xargo
	$(ARMGNU)-ld $(OBJECTS) $(OUTPUT)libbarelyos.a -o $(BUILD)kernel.elf -T $(LINK_SCRIPT)

# Rule to make the object files
$(BUILD)%.o: $(SOURCE)%.s $(BUILD)
	$(ARMGNU)-as $< -mcpu=$(CPU) -march=$(ARCH) -o $@

# Rule to make the listing file.
$(LIST): $(BUILD)kernel.elf
	$(ARMGNU)-objdump -D $(BUILD)kernel.elf > $(LIST)

# Compile the Rust code with Xargo
xargo:
	xargo build --release --target=$(CPU)

# Create the build directory
$(BUILD):
	mkdir -p $@

# Clean all the intermediate and output files
clean:
	xargo clean
