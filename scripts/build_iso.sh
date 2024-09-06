#!/bin/sh
set -e
die() {
    echo "error: $@" >&2
    exit 1
}

[ -e ./README.md ]   \
    || die "You must run this script from the root of the repository"

# Check that limine is installed and build it if necessary
if [ ! -e bin/src/limine/limine-uefi-cd.bin ] ||
   [ ! -e bin/src/limine/limine-bios-cd.bin ] ||
   [ ! -e bin/src/limine/limine-bios.sys ]; then
    echo "Limine is not installed. Downloading and building it..."
    ./scripts/build_limine.sh
fi

mkdir -p iso/boot
mkdir -p iso/EFI/BOOT

# Copy the limine configuration file
cp -v config/limine.cfg iso/boot/limine.cfg

# Copy the limine bootloader inside the ISO directory
cp -v                                   \
    bin/src/limine/limine-uefi-cd.bin   \
    bin/src/limine/limine-bios-cd.bin   \
    bin/src/limine/limine-bios.sys      \
    iso/boot/

cp -v                                   \
  bin/src/limine/BOOTAA64.EFI           \
  bin/src/limine/BOOTIA32.EFI           \
  bin/src/limine/BOOTX64.EFI            \
  iso/EFI/BOOT/

# Install the kernel
cp -v kernel/target/kernel-x86_64/release/kernel iso/boot/silicium.elf

# Create the ISO
xorriso -as mkisofs -b boot/limine-bios-cd.bin              \
		-no-emul-boot -boot-load-size 4 -boot-info-table    \
		--efi-boot boot/limine-uefi-cd.bin                  \
		-efi-boot-part --efi-boot-image                     \
		--protective-msdos-label iso -o bin/silicium.iso

# Deploy Limine to the ISO
./bin/src/limine/limine bios-install bin/silicium.iso
