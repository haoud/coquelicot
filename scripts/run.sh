#!/bin/sh
die() {
    echo "error: $@" >&2
    exit 1
}

[ -e ./README.md ]   \
    || die "you must run this script from the root of the repository"

qemu-system-x86_64                                  \
    -device isa-debug-exit,iobase=0xf4,iosize=0x04  \
    -display gtk,gl=on                              \
    -cdrom bin/silicium.iso                         \
    -rtc base=localtime                             \
    -serial stdio                                   \
    -vga virtio                                     \
    -cpu max                                        \
    -smp 2                                          \
    -m 128

code=$?
if [ $code -eq 3 ]; then
    exit 0
else
    exit $code
fi
