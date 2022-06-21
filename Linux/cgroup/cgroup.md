# Control Group (`cgroup`)

- [Control Group (`cgroup`)](#control-group-cgroup)
  - [**`cgroup`**](#cgroup)
    - [cgroup study links](#cgroup-study-links)
  - [**`cgroupv2`**](#cgroupv2)
    - [cgroup2 Study Links](#cgroup2-study-links)

## **`cgroup`**

- **What it is**
  - [c13]
    - Control groups (cgroup) are a **feature** of Linux kernel by which ***groups of processes can be monitored and have their resources limited***
      - E.g. If we don't want to control Google Chrome
        - Limiting a process not to exceed 1 GB RAM or 30% CPU usage
        - `cgroup` would let you do that.

- **Types of cgroups**
  - There are **12 different types of cgroups** available in Linux
    - Each of them
      - Corresponds to a resource that processes use

- **Limitations**
  - cgroups specifically deals with processes (which are a fundamental piece of any OS)
  - A process is just a running instance of a program
    - When you want to run a program
      - Linux kernel **loads the program into memory**
      - Assings a Process ID **PID**
      - **Allocates various resources** for it
    - **Throughout the life time of the process**
      - Kernel **keeps tracks** of its various **state and resource usage** information
  - **Subsystems**
    - Some resources that are needed by a program
      - Memory
        - To store and use data
      - CPU
        - To execute instructions
      - Access to devices
        - HDD or Keyboard for example
    - **each of these resources are abstracted by Linux Kernel** - These abstractions are called as **subsystems**
      - *Example of a subsystem is the virtaul memory management system*
    - Example for *Memory*: **`VFS Subsystem`**
      - This is the layer between the memory management unit (MMU) and rest of the Kernel
      - When running program
        - Allocates memory for new data structure (thorough something like **malloc**)
          - There is functionality in Kernel
            - to resize the heap of that process
      - All process share a single pool of memory
        - That can be allocated for each of their use
      - There is a chance that
        - An application can hog all your memory
    - Example for CPU: **`Scheduler Subsystem`**
      - CPU time is a shared resource
      - `scheduler` in the cgroup shows how cgroup works

- Where to find details
  - `cat /proc/[pid]/cgroup`

- `cpuset`
  - [c12] Confine process to processor and memory node subsets

### cgroup study links

- [c11]
  - <https://docs.oracle.com/en/operating-systems/oracle-linux/6/adminsg/ol_cgroups.html>
- [c12]
  - <https://man7.org/linux/man-pages/man7/cpuset.7.html>
- [c13]
  - <https://www.grant.pizza/blog/understanding-cgroups/>
    - Understanding cgroups
- [c14]
  - <https://www.redhat.com/sysadmin/cgroups-part-one>
    - Details how cgroup works
- [c15]
  - <https://www.youtube.com/watch?v=w9YQawAfkPw>
    - Linux Memory Management at Scale
      - Talks about cgroup

## **`cgroupv2`**

- Linux new unified contorl group system
  - For resource control and management in the Linux Kernel
- `cgroup`
  - It is a kernel mechanism to
    - Balance, Limit and Throttle **"Things"** like
      - Memory, CPU, IO
      - **Things** that you share across the system and across the applications

- **Why cgroup**
  - **To address scalability**
    - Instead of increasing more hardwares, how efficiently we can use existing systems capacity?

### cgroup2 Study Links

- [c21]
  - <https://www.youtube.com/watch?v=w9YQawAfkPw>
    - Linux Memory Management at Scale
      - Talks about cgroup
