# Debugging

- [Debugging](#debugging)
  - [Memotry Tracking Tools](#memotry-tracking-tools)
  - [Linkers and Libraries](#linkers-and-libraries)
    - [`LD_DEBUG`](#ld_debug)
    - [`LD_LIBRARY_PATH`](#ld_library_path)
    - [`LD_PRELOAD`](#ld_preload)
    - [`LD_TRACE_LOADED_OBJECTS`](#ld_trace_loaded_objects)
  - [Crash](#crash)
    - [Glibc based Backtrace](#glibc-based-backtrace)
    - [Enhanced GLibc based Backtrace](#enhanced-glibc-based-backtrace)
    - [Strace](#strace)
    - [Core Dump](#core-dump)
      - [set the maxsize of a core dump - `ulimit`](#set-the-maxsize-of-a-core-dump---ulimit)
      - [Enabling CoreDump](#enabling-coredump)
      - [Get Stack Trace from CoreDump](#get-stack-trace-from-coredump)
      - [Core Dump Study Links](#core-dump-study-links)
    - [crashinfo](#crashinfo)
    - [Minicoredumper](#minicoredumper)
    - [Breakpad](#breakpad)
    - [Custom Crash Handler](#custom-crash-handler)
  - [Memory](#memory)
    - [ASan (Address Sanitizer)](#asan-address-sanitizer)
      - [ASan Study Links](#asan-study-links)
  - [Links](#links)

## Memotry Tracking Tools

- smemcap
  - Captures memory for all processess
  - Later we can analyze
- heap Track ???
  - <https://github.com/KDE/heaptrack>

## Linkers and Libraries

- <https://datacadamia.com/os/linux/ld_library_path>
- **Many Environment variable options are listed in below link**
  - <https://www.man7.org/linux/man-pages/man8/ld.so.8.html?msclkid=c6b21779a5f011ec90faeb08ead5d04e>

### `LD_DEBUG`

- `LD_DEBUG`
  - It shows how symbols (variables and fuctions, for example) are resolved for a dynamic executable
  - Details `LD_DEBUG` with an example code
    - <https://docs.oracle.com/cd/E19120-01/open.solaris/819-0690/chapter3-33/index.html?msclkid=c192d833a5eb11ec93aec40677697f64>
  - Study Link
    - <http://www.bnikolic.co.uk/blog/linux-ld-debug.html?msclkid=c1915fbda5eb11eca03f55b3185c337c>
- **How to trace library loading**
  - <https://unix.stackexchange.com/questions/448964/how-do-i-trace-library-loading?msclkid=d6ce2863a5ef11ecbe610d3d40f0fb3b>

### `LD_LIBRARY_PATH`

- `LD_LIBRARY_PATH`
  - It lists directories where executable can search for Linux shared library
  - Also called as ***shared libary search path***

- `search_library.sh`
  - `search_library.sh libodbc.so`

    ```bash
    #!/bin/bash
    IFS=:

    for p in ${LD_LIBRARY_PATH}; do
        if [ -e ${p}/${1} ]; then
            echo ${p}
        fi
    done
    ```

### `LD_PRELOAD`

- Use LD_PRELOAD environment variable to **instructs** the loader to **load additional libraries into a program**, **beyond what was specified when it was compiled**.
  - The [LD_PRELOAD](https://www.cyberciti.biz/tips/linux-shared-library-management.html?msclkid=d6cc98e7a5ef11ecaf577154fe631c0e) allow an extra library not specified in the executable to be loaded:
    - `$ export LD_PRELOAD=/home/vivek/dirhard/libdiehard.so`

### `LD_TRACE_LOADED_OBJECTS`

## Crash

- In-built tools
  - Glibc based Backtrace
  - Enhanced GLibc based Backtrace
  - Strace
  - Coredump
  - crashinfo
- 3rd Party Tools
  - Mini Coredump
  - Breakpad

### Glibc based Backtrace

- Based on GNU and part of libgcc.
- Stores a functions return address.
- Inline Functions or Static Functions will not appear on stack frames.
- Depends on the Dynamic Section / Symbol table, since the address needs to be converted to strings in order to identify function names.
  - Symbol names can be made available by using linker options like `-g` / -rdynamic etc.

### Enhanced GLibc based Backtrace

- Base line based on [Glibc based Backtrace](#glibc-based-backtrace)
- More about improving the symbol table where many shared libraries used.
- Identifying shared libraries where we can improve the symbol table based on linker options.

### Strace

- Strace is a diagnostic and debugging utility used to monitor system calls, signal deliveries and process state.
- Strace is needed if it is not available in the target platform.
- Mainly logs the systems calls and not at an application level which reduces application level understanding for us.

### Core Dump

- A "**segfault**" or "**segmentation fault**" is when your program tries to access memory that it is not allowed to access, or tries to.

- A **core dump** is a copy of your program's memory and it is useful when you are trying to debug what went wrong with your problematic program.
- When your program segfaults, the Linux kernel will sometimes write a core dump to disk.

#### set the maxsize of a core dump - `ulimit`

- ulimit maximizes the size of a core dump
- it is often set to `0`, which means that the kernel won't write core dumps at all.
- it is in kilo bytes
- ulimits are ***per process***
  - Example

    ```bash
    / # cat /proc/1608/limits
    Limit                     Soft Limit           Hard Limit           Units
    Max cpu time              unlimited            unlimited            seconds
    Max file size             unlimited            unlimited            bytes
    Max data size             unlimited            unlimited            bytes
    Max stack size            8388608              unlimited            bytes
    Max core file size        unlimited            unlimited            bytes
    Max resident set          unlimited            unlimited            bytes
    Max processes             7575                 7575                 processes
    Max open files            512                  512                  files
    Max locked memory         65536                65536                bytes
    Max address space         unlimited            unlimited            bytes
    Max file locks            unlimited            unlimited            locks
    Max pending signals       7575                 7575                 signals
    Max msgqueue size         819200               819200               bytes
    Max nice priority         0                    0
    Max realtime priority     0                    0
    Max realtime timeout      unlimited            unlimited            us
    ```

- The kernel uses the **soft limit** when deciding how big of a core file to write.
- ***You can increase the soft limit up to the hard limit using the below command:***
  - Run `ulimit -c unlimited` before starting your program
- Run `sudo sysctl -w kernel.core_pattern=/tmp/core-%e-.%p.-%h.%t`

#### Enabling CoreDump

- `ulimit -c unlimited`
- `echo "/mnt/2-2b17f62a-8a96-4211-97b5-fbbb84024e3f/cores/%e.%s.%p.core" > /proc/sys/kernel/core_pattern`
- `./bootimage/rootfs/sysbase/debug/S11coresdir:25:  echo "/cores/coredump/core.%e.%p.%h.%t" >/proc/sys/kernel/core_pattern`

```bash
setCoresDir()
{
  dir=$1
  /bin/chmod 777 $dir/cores
  /bin/umount /cores/coredump 2> /dev/null
  /bin/mount --bind $dir/cores /cores/coredump
  echo "Redirecting core dumps to $dir/cores/"
  echo "/cores/coredump/core.%e.%p.%h.%t" >/proc/sys/kernel/core_pattern
  echo "2" >/proc/sys/fs/suid_dumpable

  fstype="$(/bin/df -T $dir | /usr/bin/awk 'NR==2 {print $2}')"
  if [ "$fstype" == "vfat" ]; then
      echo "Using a vfat partition to save core dumps is not fully supported. Core dumps might not be saved from all applications."
  fi
  clearBootCores
}
```

#### Get Stack Trace from CoreDump

- The generated core files can be used for analyzing the crash. This will help us to understand the reason for the crash and prevent it from happening in the future.
- Once the Coredump is generated on the cores directory on HDD path,

1. we need to copy it to our Build server where we have necessary tools like `gdb` to get the stacktrace and the build which was used for creating the image which was responsible for generating the core dump.
2. After copying the core file to build server, run `gdb-multiarch`,
   1. **`gdb-multiarch <executable file> <core file>`**
      1. `gdb-multiarch /mnt/home/user/workspace/xxx.bin /mnt/home/user/workspace/xxx.core`
3. Set sysroot path for finding necessary shared libraries which are required to resolve the address on core files,
   1. **`set sysroot </path/to/sysroot>`**
      1. `set sysroot /mnt/home/user/workspace/debug/target/sysroot`
   2. **`set sysroot </path/to/rootfs>`**
      1. `set sysroot /mnt/home/user/workspace/debug/images/rootfs`
4. Get stack trace
   1. **`thread apply all bt full`**
      1. This will give the stack trace for all the threads for the current core dump which may include symbols from other third party libraries too.
   2. **`bt full`**
      1. This will give stack trace for the thread which was responsible for the crash or program termination.

#### Core Dump Study Links

- <https://jvns.ca/blog/2018/04/28/debugging-a-segfault-on-linux/>

### crashinfo

- Idea is to have enhanced print of backtrace frames
- Following singals can be handled (other signals also can be handled)
  - SIGSEGV
  - SIGBUS
  - SIGILL
  - SIGFPE

### Minicoredumper

- [Microdumper](https://www.linutronix.de/minicoredumper/) be used to generate minimal and customized core dump files
- Provides highly configurable core dump mechanism
- microdumper is intended to be executed by Linux core dump facility

### Breakpad

- [Breakpad](https://chromium.googlesource.com/breakpad/breakpad/) is a set of client and server components which implement a crash-reporting system.
- Breakdpad doesn't require debugging information in the final application
- It records crashes in compact minidump files and then send them to the sever and then further produces C/C++ stack traces from these minidumps
- Minidumps can be created also on demand without a crash
- Breakpad has mainly three components
  - **Client**:
    - Library to be included in our application
  - **Symbol Dumper**:
    - Program which reads debugging information produced and creates a suymbol file
  - **Processor**:
    - Reads minidump file

### Custom Crash Handler

- Based on existing tools/utilities available we can create a custom crash handler module which can then further be utilized for any C/C++ applications for crash reporting

## Memory

- Valgrind
- Heaptrack
- ASAN

### ASan (Address Sanitizer)

- [Address Sanitizer(https://github.com/google/sanitizers/wiki/AddressSanitizer)] is a tool developed by Google
  - It is a memorry error detector for C/C++
- **It finds**
  - `User after free`
  - `Heap buffer overflow`
  - `Stack buffer overflow`
  - `Global buffer overflow`
  - `Use after return`
  - `User after scope`
  - `Initialization order bugs`
  - `Memory Leaks`
- **It is based on**
  - Compiler instrumentation
  - Directly mapped shadow memory
- **Possible overload**
  - **`73%`** increase in processing time
  - **`24%`** increase in memory usage
- **Limitations**
  - It **does not detect** any uninitialized meory reads
  - Only **detects some** user-after-return bugs
  - It is also **not capable** of detecting
    - all arbitrary memory corruption bugs
    - nor all arbitraty write bugs - due to integer underfow/overflows
      - When the integer with undefined behavior is used to calculate memory address offsets
- **GCC Support**
  - From gcc version 4.8 and above
- **Dependecy**
  - `libasan.so`
- **Its Components**
  - Compiler instrumentation module
    - an LLVM pass
  - A run-time library
    - Which replaces the `malloc` function
- **Supported platforms**
  - Refer google site
- **How it works**
  - Please read <https://github.com/google/sanitizers/wiki/AddressSanitizerAlgorithm>
    - the run-time library replaces `malloc` and `free` functions
      - `malloc` - red zone
      - `free` - quarantine zone
    - Converts

        ```c++
        `*address = ...;  // or: ... = *address;`
        ```

    - To

        ```c++
        if (IsPoisoned(address)) {
          ReportError(address, kAccessSize, kIsWrite);
        }
        *address = ...;  // or: ... = *address;
        ```

- **Required Modifications**
  - In order to enable address sanitizer library (to be part of your platform), below things need to be done:
    - Add `libasan.so` to the optional flags required within `/usr/lib` of your platform
    - Add appropriate `cflags` / `cxxflags to generate report stating the file/function name causing the issue
      - `-fsanitize=address` used for enabling address sanitizer
      - `-fno-emit-frame-pointer` used for leaving the frame pointer
    - Link the dependent library `libasan.so`
    - Create required symbolic links if any (e.g., symbolic link to `/lib` of `/usr/lib`)

#### ASan Study Links

- ***My notes are captured [here](ASAN.md)***

- <https://www.google.com/url?sa=i&url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3D1CcuD7EwhOY&psig=AOvVaw2CP6q8lomKIqo4U_EdCTxk&ust=1647587919246000&source=images&cd=vfe&ved=0CAkQjhxqFwoTCJCv4rfNzPYCFQAAAAAdAAAAABAc>
- <https://www.google.com/url?sa=i&url=https%3A%2F%2Fembeddedbits.org%2Ffinding-memory-bugs-with-addresssanitizer%2F&psig=AOvVaw2CP6q8lomKIqo4U_EdCTxk&ust=1647587919246000&source=images&cd=vfe&ved=0CAkQjhxqFwoTCJCv4rfNzPYCFQAAAAAdAAAAABAi>
- <https://www.google.com/url?sa=i&url=https%3A%2F%2Fuseyourloaf.com%2Fblog%2Fusing-the-address-sanitizer%2F&psig=AOvVaw2CP6q8lomKIqo4U_EdCTxk&ust=1647587919246000&source=images&cd=vfe&ved=0CAkQjhxqFwoTCJCv4rfNzPYCFQAAAAAdAAAAABAt>
- <https://www.google.com/url?sa=i&url=https%3A%2F%2Fmedium.com%2F%40ibnjunaid%2Faddress-sanitisers-in-c-c-29ad49a92b8&psig=AOvVaw2CP6q8lomKIqo4U_EdCTxk&ust=1647587919246000&source=images&cd=vfe&ved=0CAkQjhxqFwoTCJCv4rfNzPYCFQAAAAAdAAAAABAz>
- <https://www.google.com/url?sa=i&url=https%3A%2F%2Fdocs.tizen.org%2Fapplication%2Ftizen-studio%2Fnative-tools%2Faddress-sanitizer%2F&psig=AOvVaw2CP6q8lomKIqo4U_EdCTxk&ust=1647587919246000&source=images&cd=vfe&ved=0CAkQjhxqFwoTCJCv4rfNzPYCFQAAAAAdAAAAABA->
- <http://gavinchou.github.io/experience/summary/syntax/gcc-address-sanitizer/?msclkid=2fba6b33a5e911ec985f90c7497f2366>
  - Blog explanation with example program for each type of memory error detection

## Links

- <https://www.linutronix.de/minicoredumper/>
- <https://chromium.googlesource.com/breakpad/breakpad/>
- <https://github.com/google/sanitizers/wiki/AddressSanitizer>
  - [Sample test Code()]<https://www.sobyte.net/post/2022-01/asan-intro/?msclkid=ca7a1dd8a5e711ecb41bfc6284f86fa8>)
- <https://linux-kernel-labs.github.io/refs/heads/master/lectures/debugging.html?msclkid=d6cde7d7a5ef11ec958c73f61034f85e>
  - Some Linux debugging steps covered in it
