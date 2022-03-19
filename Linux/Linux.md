# Linux

- [Linux](#linux)
  - [Device Drivers](#device-drivers)
  - [Memory](#memory)
  - [Debugging](#debugging)
  - [Editors](#editors)
  - [System Administration](#system-administration)
  - [CTags and CScope](#ctags-and-cscope)
  - [ARM Compiled Binaries](#arm-compiled-binaries)
  - [eBPF](#ebpf)
  - [Some How To Find](#some-how-to-find)
  - [Books](#books)
  - [Study Links](#study-links)

## Device Drivers

Device driver notes can be found [here](DeviceDrivers.md)

## Memory

Linux Memory related notes can be found [here](Memory.md)

## Debugging

All debugging related to Linux can be found [here](Debugging/Debugging.md)

## Editors

`vi` info can be found [here](vi/vi.md)

## System Administration

- Monitoring and System Administration related notes can be found [here](monitoring_and_administration/Troubeshooting.md)

## CTags and CScope

- <https://hackereyes.tistory.com/entry/%ED%8E%8C-ctags-cscope-%EC%84%A4%EC%B9%98-%EB%B0%8F-%EC%82%AC%EC%9A%A9>
  - shell script

## ARM Compiled Binaries

- <https://github.com/andrew-d/static-binaries/tree/master/binaries/linux>
  - Here you can find the readly compiled binaries for Windows/Linux/Darwin for arm/x86/x86_64

## eBPF

- TO Study
  - <https://ebpf.io/>
  - <https://man.archlinux.org/man/bpftool.8.en>
  - Sample codes
    - <https://github.com/grantseltzer/trace-memory>
    - <https://github.com/grantseltzer/trace-signals>

## Some How To Find

- Which interrupts are being used by the device drivers?
  - `cat /proc/interrupts`
- How many each type of interrupts there have been?
  - `cat /proc/interrupts`
- What are Subsystems in Linux
  - Refer [cgroup](cgroup/cgroup.md) notes (especially cgroup)
- What are Things in Linux
  - Refer [cgroup](cgroup/cgroup.md) notes (especially cgroupv2)

## Books

- [1] <http://www.science.unitn.it/~fiorella/guidelinux/tlk/node5.html>
  - Very old book
  - But provides insights how we have to understand the Linux code

## Study Links

- <https://www.grant.pizza/>
  - Has some good blog on Linux
    - cgroup
    - vmlinux.h
    - DWARF
    - procfs
    - ...
