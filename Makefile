ARCH ?= x86_64
MODE ?= debug
qemu := qemu-system-$(ARCH)
target := $(ARCH)
build_path := target/$(target)/$(MODE)
ESP := $(build_path)/esp
OVMF := OVMF.fd

# -serial mon:stdio + -nographic
run:
	qemu-system-x86_64 \
	-drive if=pflash,format=raw,readonly,file=OVMF.fd \
	-drive format=raw,file=fat:rw:target/x86_64-unknown-uefi/debug \
	-m 4G \
	-device isa-debug-exit \
	-net none