PROFILE	?= debug
BUILDDIR:= target/$(PROFILE)

ARCH	?= x86_64
KERNEL	:= $(BUILDDIR)/kernel-$(ARCH).bin
ISO		:= $(BUILDDIR)/kernel-$(ARCH).iso

LINKFILE:= src/link.ld
ISODIR	:= isofiles
ASMSRC	:= $(wildcard src/arch/$(ARCH)/*.asm)
ASMOBJ	:= $(patsubst src/arch/$(ARCH)/%.asm, $(BUILDDIR)/%.o, $(ASMSRC))
RUSTLIB	:= target/$(BUILDDIR)/librust_os.a # yes, it does that )=

.PHONY: all run clean iso dirs_setup cloc

all: run

clean:
	@rm -r target/ || true

dirs_setup: clean
	@mkdir -p $(BUILDDIR)
	@cp -r $(ISODIR) $(BUILDDIR)

run: $(ISO)
	qemu-system-$(ARCH) -cdrom $(ISO)

iso: $(ISO)
	@cp $(ISO) target/
	@echo Done!

kernel: $(KERNEL)
	@cp $(KERNEL) target/
	@echo Done!

rust: dirs_setup $(RUSTLIB)
	@cp $(RUSTLIB) target/
	@echo Done!

$(ISO): dirs_setup $(KERNEL)
	cp $(KERNEL) $(BUILDDIR)/$(ISODIR)/boot/kernel.bin
	grub-mkrescue -o $(ISO) $(BUILDDIR)/$(ISODIR)

$(KERNEL): dirs_setup $(RUSTLIB) $(ASMOBJ) $(LINKFILE)
	ld -n -o $(KERNEL) -T $(LINKFILE) $(ASMOBJ) $(RUSTLIB)

$(RUSTLIB):
	cargo xbuild --target=src/arch/$(ARCH)/target.json

$(BUILDDIR)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	nasm -felf64 -DVIDEO $< -o $@

cloc:
	@cloc . --exclude-dir=target