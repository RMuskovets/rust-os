PROFILE	?= debug
BUILDDIR:= target/$(PROFILE)

ARCH	?= x86_64
KERNEL	:= $(BUILDDIR)/kernel-$(ARCH).bin
ISO		:= $(BUILDDIR)/kernel-$(ARCH).iso

LINKFILE:= src/link.ld
ISODIR	:= isofiles
ASMSRC	:= $(wildcard src/arch/$(ARCH)/*.asm)
ASMOBJ	:= $(patsubst src/arch/$(ARCH)/%.asm, $(BUILDDIR)/%.o, $(ASMSRC))

.PHONY: all run clean iso dirs_setup

all: run

clean:
	@rm -r $(BUILDDIR)

dirs_setup: clean
	@mkdir -p $(BUILDDIR)
	@cp -r $(ISODIR) $(BUILDDIR)

run: $(ISO)
	qemu-system-$(ARCH) -cdrom $(ISO)

$(ISO): dirs_setup $(KERNEL)
	cp $(KERNEL) $(BUILDDIR)/$(ISODIR)/boot/kernel.bin
	grub-mkrescue -o $(ISO) $(BUILDDIR)/$(ISODIR)

$(KERNEL): dirs_setup $(ASMOBJ) $(LINKFILE)
	ld -n -o $(KERNEL) -T $(LINKFILE) $(ASMOBJ)

$(BUILDDIR)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	nasm -felf64 $< -o $@
