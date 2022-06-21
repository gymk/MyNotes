# Memory

- [Memory](#memory)
  - [Drop VM Caches](#drop-vm-caches)
  - [Check System Memory](#check-system-memory)
  - [Address space of a Process](#address-space-of-a-process)
  - [Virtual Memory](#virtual-memory)
  - [Tools](#tools)
  - [Memory pressure notification](#memory-pressure-notification)
  - [Process Memory Management](#process-memory-management)
    - [malloc_trim](#malloc_trim)
  - [Debugging](#debugging)
  - [Study Links](#study-links)

- Tutorial: Beginners guide on linux memory management
  - <https://www.golinuxcloud.com/tutorial-linux-memory-management-overview/>
- Linux Memory Usage
  - <https://fritshoogland.wordpress.com/2017/07/25/linux-memory-usage/>
  - Script
    - <https://gitlab.com/FritsHoogland/memstat/-/blob/master/memstat.sh>

## Drop VM Caches

- Why drop caches in Linux?
  - <https://serverfault.com/questions/597115/why-drop-caches-in-linux>
- Modern Memory Management
  - <https://thedailywtf.com/articles/Modern-Memory-Management>
- Good Q&A style info on RAM in Linux
  - <https://www.linuxatemyram.com/>

- Example Steps

  ```bash
  #!/bin/bash 
  err="Not enough RAM to clear swap." 
  sync 
  echo 1 > /proc/sys/vm/drop_caches 
  echo 2 > /proc/sys/vm/drop_caches 
  echo 3 > /proc/sys/vm/drop_caches 
  mem=`free|grep Mem:|awk '{print $4}'` 
  swap=`free|grep Swap:|awk '{print $3}'` 
  test $mem -lt $swap && echo -e $err && exit 1 
  ```

## Check System Memory

- `free | grep Mem: | awk '{print $4}'`
- `free -m`
  - In below example, if we look at `used` and `free`, it is 99%, but reality is 47% it is full (see the `available` )

    ```bash
      $ free -m
                    total        used        free      shared  buff/cache   available
      Mem:           1504        1491          13           0         855      792
      Swap:          2047           6        2041
    ```

- /proc/meminfo
  - <https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=34e431b0ae398fc54ea69ff85ec700722c9da773>
    - TO READ

## Address space of a Process

- pmap
  - display information about the address space of a process
  - <https://docs.oracle.com/cd/E19683-01/816-0210/6m6nb7mhj/index.html>
    - `pmap -x 2183`
    - `Address   Kbytes     PSS   Dirty    Swap  Mode  Mapping`
    - `00010000      40      40      40       0  r-xp  /bin/zpk_wrapper`
    - `00029000       4       4       4       0  rw-p  /bin/zpk_wrapper`
    - `0084f000     148      44      44       0  rw-p  [heap]`
    - `b5b8b000      24       1       0       0  r-xp  /lib/libnss_files-2.24.so`
    - ...
    - --------  ------  ------  ------  ------
    - total      20332     360     312       0

- Anon pages
  - <https://jike.in/?qa=777097/java-trying-to-locate-a-leak-what-does-anon-mean-for-pmap>
    - Anon blocks are "large" blocks allocated via `malloc` or `mmap`.
    - Threads also use anon blocks.
    - If you see thousands of blocks,it means you have a problem with threading

## Virtual Memory

- vmstat
  - Virtual memory statistics

## Tools

- SAR in Linux
  - System performance monitoring tool
  - <https://unix.stackexchange.com/questions/362833/how-to-trigger-action-on-low-memory-condition-in-linux/362834>
- Nagios
  - <https://en.wikipedia.org/wiki/Nagios>

## Memory pressure notification

- <https://unix.stackexchange.com/questions/362833/how-to-trigger-action-on-low-memory-condition-in-linux/362834>
  - <https://www.kernel.org/doc/Documentation/cgroup-v1/memory.txt>
    - register an eventfd file descriptor in
      - `/sys/fs/cgroup/memory/memory.pressure_level` on which you want to receive notifications.
      - These notifications can be `low`, `medium`, and `critical`
  - Use Case
    - To free some or all internal caches in your process when you receive a notification, in order to prevent an impending OOM kill

## Process Memory Management

- <https://www.baeldung.com/linux/process-memory-management>
  - Details VSZ, RSS
  - VSZ - Virtual Memory Size
    - Total amount of memory a process may hypothetically access
      - It accounts
        - The size of the binary itself
        - any linked libraries
        - any stack or heap allocations
    - When
  - RSS - Resident Set Size

- [s1]
  - Memory management is actually a ***trade-off between performance and accuracy***

### malloc_trim

- <https://man7.org/linux/man-pages/man3/malloc_trim.3.html>
  - Release free memory from heap

## Debugging

- Refer [here](Debugging/Debugging.md)

## Study Links

- [s1]
  - <https://www.youtube.com/watch?v=w9YQawAfkPw>
    - Linux Memory Management at Scale
      - Talks about cgroup
