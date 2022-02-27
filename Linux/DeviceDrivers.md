# Device Drivers

- [Device Drivers](#device-drivers)
  - [**Linux Device Driver**](#linux-device-driver)
    - [**Device Representation**](#device-representation)
    - [**Types of devices**](#types-of-devices)
    - [**Device Driver common attributes**](#device-driver-common-attributes)
  - [Polling and Interrupts](#polling-and-interrupts)
    - [Polling](#polling)
    - [Interrupts](#interrupts)
  - [DMA (Direct Memory Access)](#dma-direct-memory-access)
  - [Books](#books)
  - [Other Links](#other-links)

## **Linux Device Driver**

- From [1]
  - The Linux kernel device drivers are, essentially, a shared library of privileged, memory resident, low level hardware handling routines
  - It is Linux's device drivers which each handle the peculiarities of the devices that they are managing

### **Device Representation**

- From [1]
  - **device special file**
    - Every device in the system is represented by a *device special file*
      - For example,
        - the first IDE disk in the system is represented by `/dev/hda`
    - **device major and minor number**
      - These device special files are created by the `mknod` command
        - and they describe the device using ***major and minor device numbers***
      - All devices controlled by the *same device driver* have a *common major number*
        - The minor numbers are used to distinquish between different devices
          - For example,
            - each partition on the primary IDE disk has a different minor device number
            - So `/dev/hda2`, the second partition of the primary IDE disk has a major number `3` and a minor number `2`.
    - **device as files**
      - One of the basic features of Unix  is that it abstracts the handling of devices.
        - *All hardware devices look like regular files*
          - They are
            - opened
            - read and
            - written
          - using the same, standard system calls, that are used to manipulate files
      - Linux VFS (virtual file system) maps the device special file passed in the systems to the device's device driver using the major device number

### **Types of devices**

- From [1]
  - Linux supports 3 types of hardware devices
    - character
    - block and
    - network
  - **character devices**
    - Character devices are *read and written directly witout buffering*
      - For example,
        - the system's serial ports `/dev/cua0` and `/dev/cua1`
  - **block devices**
    - Block devices can only be *written to and read in multiple of the block size*
      - typically 512 or 1024 bytes
    - Block devices are *accessed via the buffer cache* and may be randomly accessed
      - That is any block can be read or written no matter where it is on the deivce
    - Block devices can be accessed via their device file but more *commonly they are accessed via the filesystem*
    - *Only a block device can support a mounted file system*

### **Device Driver common attributes**

- From [1]
  - There are many different device drivers in the Linux kernel but they all share some common attributes:
    - **kernel code**
      - Device drivers are part of the kernel and, like other code within the kernel, if they go wrong they can seriously damage the system. A badbly written driver may even crash the system, possibly corrupting file systems and losing data
    - **Interfcaes**
      - A device driver must provide a standard interface to the kernel.
      - The kernel uses this interface to interact with the devices that this device driver controls.
        - That interface is different depending on the class of device driver
          - So, for example,
            - the interface provided by a SCSI device is different from that provided by an Ethernet device.
      - The kernel may also provide access to the system devices to processes in the system via standard interfaces.
        - For example,
          - the socket interface can be used to connect to the system's ethernet devices
    - **Kernel services**
      - Device drivers make use of standard kernel services
        - such as
          - memory allocation
          - interrupt delivery and
          - wait queues to operate
    - **Loadable**
      - Most of the Linux device drivers can be loaded on demand as kernel modules when they are needed and unloaded when they are no longer being used.
        - This makes the kernel very adaptable and efficient with the system's resources
    - **Configurable**
      - Linux device drivers can be built into the kernel
        - Which devices are built is configurable when the kernel is compiled
    - **Dynamic**
      - As the system boots and each device driver is initialized it looks for the hardware devices that it is controlling
        - It does not matter if the device being controlled by a particular device driver does not exist.
          - In this case, the device driver is simply redundant and causes ho harm apart from occupying a little of the system's memory

## Polling and Interrupts

- From [1]
  - For any command given to a device driver, it has two choices to complete the command
    - The device driver can
      - *Either* **poll the device**
      - *Or* they **can use interrupts**

### Polling

- From [1]
  - Polling the device usually means reading its status register every so often until the device's status changes to indicate that it has completed the request.
    - **Continuous polling is disastrous**
      - As device driver is part of the kernel it would be *disastrous if a driver were to poll* as nothing else in the kernel would run until the device had completed the request.
    - **Use system timers for polling**
      - Instead, polling device drivers use system timers to have the kernel call a routine within the device driver at some later time
        - This timer routine would check the status of the command
    - **Polling using timer is approximate**
      - Polling uses timers at its best approximate, ***a much more efficient is to use interrupts***.

### Interrupts

- From [1]
  - An interrupt device drvier is one where the hardware device being controlled will cause a hardware interrupt to occure whenever it needs to be served.
    - For example,
      - An ethernet device driver would interrupt whenever it receives an ethernet packet from the network.
  - **Registering interrupt**
    - The Linux kernel needs to be able to deliver the interrupt from the hardware device to the correct device driver.
      - This is acheived by the device driver registering its usage of the interrupt with the kernel.
        - It **registers** the ***address of an interrupt handling routine*** and ***the interrupt number*** that is wishes to own.
      - The requesting of interrupt resources is done *at driver initialization time*.
  - **Interrupt delivery**
    - How an interrupt is delivered to the CPU itself is architecture dependent
      - But on most architectures the interrupt is delivered in a special mode that stops other things from happening in the system.
  - **Caution in interrupt routine**
    - A device driver should do as little as possible in its interrupt routine
      - So that Linux kernel can dismiss the interrupt and return to what it was doing before it was interrupted.
    - **Bottom half**
      - Device drivers that need to do a lot of work as a result of receiving an interrupt can use kernel's **bottom half handlers** or **task queues** to queue routines to be called later on.

## DMA (Direct Memory Access)

## Books

- [1] <http://www.science.unitn.it/~fiorella/guidelinux/tlk/node5.html>
  - Very old book
  - But provides insights how we have to understand the Linux code

## Other Links

- <https://blog.naver.com/ultract2/221250421474>
  - `ctags` and `cscope` configuration
- <https://courses.cs.washington.edu/courses/cse451/10au/tutorials/tutorial_ctags.html>
  - Ctags tutorial
- <https://jen6.tistory.com/119>
  - Vim, Ctags, CScope
- <https://ndesh26.github.io/programming/2018/07/27/A-Personal-Guide-to-Linux-kernel-Makefile/>
  - A Personal Guide to Linux kernel's Makefile
    - menuconfig
    - cscope
    - building specific modules
    - defconfig
    - htmldocs
    - localmodconfig
    - obvious commands
  - <https://www.embeddedts.com/blog/tag-jumping-in-a-codebase-using-ctags-and-cscope-in-vim/>
    - Tag Jumping in a Codebase Using ctags and cscope in Vim
