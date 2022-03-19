# Linux System Maintenance and Troubleshooting

- [Linux System Maintenance and Troubleshooting](#linux-system-maintenance-and-troubleshooting)
  - [Monitoring and Troubleshooting Linux System Performance](#monitoring-and-troubleshooting-linux-system-performance)
    - [Identifying System Resource Usage (System Metrics)](#identifying-system-resource-usage-system-metrics)
      - [**`proc`**](#proc)
      - [**`journalctl`**](#journalctl)
      - [**`cpuinfo`**](#cpuinfo)
      - [**`uptime`**](#uptime)
      - [**`filesystems`**](#filesystems)
      - [**`meminfo`**](#meminfo)
      - [**`free`**](#free)
    - [Storage and Network Bandwidth Metrics (System Metrics)](#storage-and-network-bandwidth-metrics-system-metrics)
      - [Stoage](#stoage)
        - [**`df`**](#df)
        - [**`inode`** limits](#inode-limits)
        - [**`top`**](#top)
      - [Network Bandwidth](#network-bandwidth)
        - [**`iftop`**](#iftop)
        - [**`nethogs`**](#nethogs)
    - [System Monitoring Solutions](#system-monitoring-solutions)
      - [Monitoring solutions for Linux](#monitoring-solutions-for-linux)
    - [Controlling System Resource Usage](#controlling-system-resource-usage)
      - [**`nice`**](#nice)
      - [**`cgroup`**](#cgroup)
    - [Defining a Capacity Planning Stratergy](#defining-a-capacity-planning-stratergy)
  - [Building a Disaster Recovery Plan](#building-a-disaster-recovery-plan)
    - [BCP (Business Continuity Plan)](#bcp-business-continuity-plan)
      - [Safety](#safety)
      - [Restore Service](#restore-service)
      - [Restore All Operation](#restore-all-operation)
      - [Incident Management Protocol (IMP)](#incident-management-protocol-imp)
      - [Disaster Recovery Plan (DRP)](#disaster-recovery-plan-drp)
  - [Maintaining System Integrity](#maintaining-system-integrity)
    - [Finding h/w and s/w](#finding-hw-and-sw)
      - [Kernel details `uname`](#kernel-details-uname)
      - [**Enumerating h/w details**](#enumerating-hw-details)
        - [**`dmidecode`**](#dmidecode)
        - [**`lshw`**](#lshw)
    - [Testing Your Hardware Integrity](#testing-your-hardware-integrity)
      - [RAM test](#ram-test)
        - [**`memtester`**](#memtester)
      - [Storage Drives test](#storage-drives-test)
        - [**`SMART`** utility](#smart-utility)
        - [Check connected drives](#check-connected-drives)
    - [Controlling Linux Kernels](#controlling-linux-kernels)
      - [Loading alternate kernel versions](#loading-alternate-kernel-versions)
        - [**How to select**](#how-to-select)
        - [**Remove older kernels**](#remove-older-kernels)
        - [**Upgrading kernels**](#upgrading-kernels)
  - [Building Software Packages for Linux](#building-software-packages-for-linux)
    - [Build Packages](#build-packages)
      - [Debian](#debian)
    - [Creating an RPM S/W Package](#creating-an-rpm-sw-package)
  - [Tools](#tools)
  - [Study Links](#study-links)

- How to care and feed your linux services need
- How you are going to give it to them

- Disaster Recovery
  - What is recovery?
  - What will your recovery require?
  - How will you communicate your plan?

- Assess key system performance metrics
- Audit the health of your hardware
- Control Linux kernel releases
- Prepare software packages for distribution
- Build a disaster recovery plan

- Commands tested on [version](images/Test_Linux_Version_1.png)

## Monitoring and Troubleshooting Linux System Performance

- Powerfull tools and well-defined practices exists for this
  - Assess capacity of system resources
    - CPU, Memory, Storage space, inode, and network bandwidth capacity and usage
  - Monitor your system (various solutions)
  - Control processess
    - How to control and how to tigtly control the access to resources given to processes
  - Planning to meet the needs
    - Building a capacity planning stratergy

  - Prepare a disaster recovery plan
    - To help face natural and human-based disasters and survive them
  - Ensure hardware functionality
    - Understand the way Linux kernel works
    - (how to) Identify immnent hardware failures
  - (how to) Build software packages

### Identifying System Resource Usage (System Metrics)

- (how to) Assess the current state of the system
- Compile a resource inventory
  - (how to) generate an inventory of our resources
    - Both
      - Absolute **total**
      - What is actually **available** right now
    - CPU, Memory, Disk space, inodes limits and network bandwidth

#### **`proc`**

- **`/proc`** has all the system information that are required
  - It is [made up of files](images/proc_folder_1.png)
  - That are generated at runtime describing the state of live processes and some system resources
  - ***numbered directory***
    - Are processes
      - `ps aux | grep 1362`
        - This [shows](images/ps_aux_grep_1.png) process owner and uptime, and the binary it is based on
  - Peeking into a [process direcotry](images/process_proc_contents_1.png)
    - Some common files can be found in each process directory
      - [`cgroup`](images/cat_cgroup_1.png), [`status`](images/cat_process_status_1.png)
    - Others will be unique to that particular process
- Example: List the files (no directories)
  - `ls -p | grep -v / | column`

#### **`journalctl`**

- **`journalctl`**
  - Can be used to learn about the what those processes have been up to lately
    - [`journalctl _PID=1362`](images/journalctl_1.png)
      - This will get you any log entris associated with the process
  - Important tool to dig (quickly) deeper into a problem

#### **`cpuinfo`**

- [`less /proc/cpuinfo`](images/less_cpuinfo_1.png)
  - Can find
    - CPU speed
    - number of cores
    - flag section
      - Gives us a long list of the capabilities built into the CPU
      - `svm` flag tells us hardware virtualization that is supported
      - `lm` tells that the CPU runs a 64-bit architecture
    - bugs section
      - Tells about which hardware vulnerabilities affect this particular chipset
      - Example `spectre`
- `less cpuinfo | grep processor`

#### **`uptime`**

- [`cat uptime`](images/uptime_1.png)
  - It shows two numbers
    - Number of seconds since the most recent boot
    - Sum of seconds your cores have been idle
- [`uptime](images/uptime_1.png)
  - Provides human readable version of the time since your last boot
  - CPU load average over the past `1`, `5`, and `15` minutes respectively

#### **`filesystems`**

- [`cat filesystems`](images/cat_filesystems_1.png)
  - This file contains all the possible file systems that your kernel supports

#### **`meminfo`**

- [`cat meminfo](images/cat_meminfo_1.png)
  - This file contains complete information about memory of the system
    - `MemTotal` represents the total RAM installed on the system (in kilobytes)
    - `MemFree` represents RAM that is currently not in used by any process (even for disk cache)
    - `MemAvailable`shows all memory that could be made available on request for new processes
- [Interpreting `/proc/meminfo`](https://access.redhat.com/solutions/406773)

#### **`free`**

- [`free`](images/cmd_free_1.png)
  - This command also gives memory status

### Storage and Network Bandwidth Metrics (System Metrics)

#### Stoage

##### **`df`**

- [`df`](images/cmd_df_1.png)
  - ***Displays each disk partition*** recognized by your system
    - Along with its ***total, used and available sizes***
  - Shows each partition's ***mount points***
- **snap** parittion
  - In recent distros, `snap` creates virtual partitions for each package
    - For security and process efficiency perspective
- Examples:
  - `df -t ext4` to limit output to one particular file system type
  - `df | grep -v snap` exclude snap partitions to get real partitions

##### **`inode`** limits

- `inode`
  - It is a metadata that is associated with each file system object that describes the object and its location
- [`df -i | grep -v snap`](images/cmd_df_inodes_1.png)
- **Gotcha**
  - Even if you have plenty of space left on your disk
  - If youhave got many thousands of very small files
  - Then you could theoretically run out of inodes

##### **`top`**

- All above metrics are read and updated every second with `top` utility

#### Network Bandwidth

- Two ways to loot at network traffic
  - From outside the box
    - **`iftop`**
  - From within it
    - **`nethogs`**

##### **`iftop`**

- Gives
  - bird's eye view of packets going in and out of your machine
- Doesn't tells you
  - Anything about the internal accounts and processes that generate the traffic
- Example:
  - `sudo iftop`

##### **`nethogs`**

- Provides
  - Who and What inside your system is making network traffic
- Example:
  - `sudo nethogs enp0s3`

### System Monitoring Solutions

- Above tools are are first-level go to tools to troubleshoot once issue occurs
- To [**monitor**](images/monitoring_solutions_intro_1.png)
  - Need an efficient way to organize and arrange useful performance metrics in a way that human beings can easily read, digest, and absorb them at regular intervals
    - Interactive Graphs
    - Charts
    - Loud and annorying alerts

#### Monitoring solutions for Linux

- Lots of [monitoring tools available for Linux](images/monitoring_sols_for_linux_1.png)
  - As Linux is in business of Servers for decades
    - [`collectd`](https://collectd.org/)
      - Remote browser-based data collection
        - Collects data from one or multiple clients and exposes the data to a browser-based interface
    - [`nagios`](https://www.nagios.org/)
      - Heavily customizable
        - [Nagios community](https://exchange.nagios.org/) has more than 800 community provided add-ons available
        - Can keep watch
          - on all kinds of devices
            - including
              - routers
              - network services
              - printers
              - windows machines
    - [`munin`](https://munin-monitoring.org/)
      - Forensics analysis
        - Focuses on historical analysis than just monitoring
          - *This makes it more useful for event forensics than ongoing monitoring*
    - [`nmon`](http://nmon.sourceforge.net/pmwiki.php)
      - SSH based visualizationzs
        - Simplie to use
        - Gives helpful performance visualizations from remote servers
          - through your regular SSH sessions
    - [`cockpit`](https://cockpit-project.org/)
      - New tool for linux server monitoring
        - Easy to install
          - `sudo apt install cokcpit`
          - `systemctl start cockpit`
          - `systemctl enable --now cockpit.socket`
          - In browser, `IP:9090`
        - browser-bsaed
        - Has ability to not only monitor, but actually to administrate the client service you are watching

### Controlling System Resource Usage

- Tools like `ps` and `nethogs` to identify running processes is important for practical reasons
- `systemd` is used in most of latest linxu distros
  - `systemctl` is used to interact with `systemd`
- To know **status of a know process** say `apache2`
  - `systemctl status apache2`
- To **limit a process to current session only**
  - `systemctl disable apache2`
    - `apache2` process will continue to run till the duration of current runing session
    - But it will not load next time the computer boots
- To **load a process at each boot**
  - `systemctl enable apache2`
- To **stop a process**
  - Method 1
    - Kill
      - `yes > /dev/nul &`
      - `yes > /dev/nul &`
      - `yes > /dev/nul &`
      - `kill 3354` using the `yes` process PID to kill it
      - `killall yess` to kill all using name of the process
  - Mehthod 2
    - `systemctl stop`
- **Contorlling**
  - Many ways it can be, some of are:
    - `nice`
    - `cgorup`

#### **`nice`**

- Instead of killing a process, we can limit it
  - We can use `nice` to limit the resources a particular process gets in relation to other running processes

#### **`cgroup`**

- `cgroup` can be used to set hard limit

### Defining a Capacity Planning Stratergy

- [**Anticipate change in capacity needs**](images/anticipate_change_in_demand_1.png)
  - Incresing compute demand
    - Is demand for your application likely to grow over the next 12 months?
  - Increasing storage capacity demand
    - Are you going to have enough storage space to hold the volume of data you are required to archive one or five years down the road>?
  - Decreasing demand?
    - Will the upcoming shutdown of a legacy service you provided mean you will have a significant and expensive amount of unused capacity?

- **System administrator is responsible** for ensuring that
  - infrastructure is able to function efficiently
  - respond to changing demand
  - delivery cost-effective service

- [**Define your current baseline performance**](images/define_you_current_baseliine_perf_1.png)
  - Assess what you have got right now
    - by establishing a usage baseline
  - Use above tools to **understand the *peak, minimal and average* usage patterns**
  - [Demand spike](images/demand_spike_1.png)
    - Even if on averae, your application demans the full resources of say 5 applications and 2 database servers
      - That won't help you when demand spike 2 or 3 times a week (and that requires to double the resources)
    - If 95% of the time only 2 or 3 servers are in use
      - Then you might consider sharing server capacity associated with other projects to make up the capacity you need during those *rare spikes*
  - **Solid monitoring system** like `cockpit` or [`sysstat`](https://github.com/sysstat/sysstat) is **required**

## Building a Disaster Recovery Plan

- There are (below) exists to address the problem of disaster recovery (one way or another)
  - [goverment regulations](fema.gov/disaster/disaster-recovery-reform-act-2018#:~:text=These%20reforms%20acknowledge%20the%20shared,for%20the%20next%20catastrophic%20event.),
  - [industry standards](iso.org/standard/44374.html)
  - Books
  - Certifications
- **Disaster Recovery** to be **planned by System Administrator** as part of Linux administration

### BCP (Business Continuity Plan)

- A formal plan meant to formalize the procedures an organization would use to ensure survival in the event of an emergency
  - In includes followig [sub-plans](images/bcp_1.png)
    - Safety
    - Restore Service
    - Restore All Operations
    - Incident Management Protocol (a sub-plan part of IT)
    - Disaster Recovery Plan (a sub-plan part of IT)

#### Safety

- Immediate safety of the employees and customers

#### Restore Service

- Work to restore previously designated critical operations as soon as possible

#### Restore All Operation

- Restore full normal operations

#### Incident Management Protocol (IMP)

- Meant to address the specific threat of cyberattacks against IT infrastructure
- Its [goal](images/incident_mgmt_plan_1.png) are
  - Minimuze damage
  - Remove the threat
- Some overlap exists between DRP and IMP
  - Key focus of
    - DMP is to get your infrastructure back on its feet
    - IMP is much more closely aligned with the world of IT security

- Usual steps
  - Alert: something is not right
  - Initiate ticket on SIEM (or Jira) event
  - Coordinate with all relevant parties
  - Close and review

#### Disaster Recovery Plan (DRP)

- Aims to protect an organization's IT infrastructure in the event of a disaster
- Its [primary goal](images/disaster_recovery_plan_1.png) are
  - Minimize damage and to
  - Restore functionality as quickly as possible
- Serious prior preparation required to have this plan working when the event occurs
  - Some **critical parts of the plan**
    - Infrastructure protection
    - Thread detection
    - Corrective protocols

- Elements of DRP
  - What is Recovery?
  - What will your Recovery Require?
  - How will you communicate your Plan?

## Maintaining System Integrity

- Maintaining hardware and operation system level integrity
  - **Why**
    - Will they all boot up tomorrow morning?
    - Did the latest overnight update break anything important?
  - **If not**
    - If you don't worry about the bad stuffs before it happens
      - You will have much more to worry about afterwards
  - **How**
    - Regularly monitor the health of your
      - storage devices
      - memory modules
    - Look for early signs of hardware failure
    - Keeping an eye on kernel updates
      - So you can protect your core application operations

### Finding h/w and s/w

#### Kernel details `uname`

- Used to identify the kernel release that is [currently live on your system](images/Test_Linux_Version_1.png)

#### **Enumerating h/w details**

- Two tools
  - `dmidecode`
  - `lshw`

##### **`dmidecode`**

- DMI
  - Desktop Management Interface
  - In older days
    - This is a framework that generated system's hardware profile
  - Recently
    - SMBIOs (System Management BIOS) performs that
    - In Linux, still `dmidecode` can be used to view system hardware details
- Tool for dumping a computer's DMI (BIOS/UEFI) table contents in a human reader format
- Examples:
  - `sudo dmidecode`
  - `sudo dmidecode -q | less`
  - `sudo dmidecode -s chassis-type`
    - E.g., `Desktop`
  - `sudo dmidecode -s baseboard-product-name`
    - E.g., `PRIMIE B450M-A`
  - `sudo dmidemocde -s processor-frequency`
    - E.g., `3500 MHz`
  - `sudo dmidecode -t bios | less`
    - Gives everything we need to know about our current BIOS version and the configurations it supports
  - `sudo dmidecode -t slot | less`
    - Gives information on the available PCIe slots

##### **`lshw`**

- lshw
  - list hardware
- It **polls virtual file systems for its information**
  - `/proc` and `/sys` directories
- Examples:
  - `sudo lshw | less`
  - [`sudo lshw -C disk`](images/sudo_lshw_C_disk_1.png)
    - Gives your device's file system mount point
  - [`sudo lshw -C network`](images/sudo_lshw_C_network_1.png)
    - Provides data on all network interfaces
      - Wired
      - Wireless
      - Virtual
  - [`sudo lshw -short`](images/sudo_lshw_short_1.png)
    - Gives quick summary of what is available (organized by class)

### Testing Your Hardware Integrity

- Checking the health of our system memory and storage drives

#### RAM test

##### **`memtester`**

- RAM test can't be accurate
  - Testing RAM can't be done accurately when the RAM is being used
  - At the same time, you can't do it if the computer is not running :-)
- Optimal way
  - Perform the test before Linux kernel loads
    - Usually in bootloader
      - [GRUB shell](images/grub_shell_1.png)
    - To install memtest in GRUB
      - `Advanced Options` -> Select `recovery mode` -> enable `network` (to download the package) -> select `root` -> install package `apt install memtester`
- [`free -h`](images/root_free_h_available_mem_1.png)
  - Check how much RAM available - needed to specify to `memtester`
- `memtester 1500 1`

#### Storage Drives test

##### **`SMART`** utility

- SMART
  - self-moniotoring, analysis, and reporting technology
  - **A tool which provides advanced warning**
    - E.g., to replace a dying drive before it goes down
- It is a standard for
  - discovering the potential for predictable hardware failures (before the failure actually happens)
- Installation
  - `sudo apt install smartmontools`
- If `SMART support is: Unavailable - device lacks SMART capability.` you can't run on the device
- Example:
  - [`sudo smartctl -i /dev/sda`](images/sudo_smartctl_i_dev_sda_1.png)
    - We can use `lsblk` to find the device (not the partition)
  - `sudo smartctl -A /dev/sda`
    - Shows us all the attributes avaiable on this drive
  - `sudo smartctl -l selftest /dev/sda`
    - Previous tests run results can be accessed with this command
  - `sudo smartctl -l xerror /dev/sda`
    - Previous tests run results can be accessed with this command
  - `sudo smartctl -l devstat /dev/sda`
    - Gives some important insights into device's history (handy for SSD when it has a failure)
      - Example:
        - Number of times device has been booted
        - Power on Hours
        - Number of write commands executed
        - Detailed information about temperature related events
  - `sudo smartctl -t short /dev/sda`
    - A short test, will take time, run below command later to get the results
      - `sudo smartctl -l selftest /dev/sda`
      - `sudo smartctl -l xerror /dev/sda`

##### Check connected drives

- [`lsblk`](images/lsblk_1.png)
  - Devices listed with `sd*`

### Controlling Linux Kernels

- [Kernel](images/linux_kernel_1.png)
  - It is a software layer that connects your user space applications to a computer's hardware resources and the peripherals you have got plugged in
  - Technically kernel is The Linux
- Consists of two parts
  - The kernel itselfs
  - [Modules](images/linux_kernel_2.png)
    - That can be added to the kernel on demand
      - Forcing all functionality into the core kernel would result in unncessary large kernel that would require rebuilding any time you wanted to edit it
      - Permitting separate modules means
        - You are able to add or remove functionality (like device drivers) on the fly without having to reboot the system

#### Loading alternate kernel versions

- Using `uname -a` we can identify which version we are already on
- **[Older versions](images/kernel_older_versions_1.png)**
  - Older versions are stored in `/boot`
- **When a new kernel installed**
  - GRUB bootloader settings are updated automatically
  - Entires to older versions will be added to the GRUB menu
  - This *enables choosing between all available kernel versions at boot time*
- **When we use (normally) older kernel versions**
  - If an application stops performing properly after a recent software update
  - We can suspect that there might be something in the update that is to blame
    - This can be confirmed by booting to previous kernel version and testing your software
- **No impact on switching kernel**
  - Switching kernel won't have any direct impact on your working file system
    - No need to worry aout losing changes that you made to files

##### **How to select**

- GRUB Menu -> Advanced options -> Select required kernel
- **Controlling GRUB options**
- GRUB items and options can be controlled through the file system
- `sudo vi /etc/default/grub`
  - GRUB behaviour related configurations
  - Default kernel version that need to be booted
  - After editing, run `update-grub`
- `cd /etc/default/grub`
- **GRUB settings**
  - `/boot/grub/grub.cfg` file
    - This is generated automatically based on the settings that are set in `/etc/default/grub`

##### **Remove older kernels**

- *CentOS*
  - `rpm -q kernel`
    - This shows how many kernel versions available
  - `yum instlal yum-utils`
    - Required to remove older kernels
  - `package-cleanup --oldkernel --count=2`
    - Requesting to leave last 2 recent versions of kernel
- *Ubuntu*
  - `apt --purge autoremove`

##### **Upgrading kernels**

- *CentOS*
  - `yum install kernel kernel-tools kernel-tools-libs`
- *Ubuntu*
  - `apt update`
  - `apt upgrade`

## Building Software Packages for Linux

- A big part of administrating Linux machines (especially remote machines), is managing and installing software
  - When something goes wrong with the local application
  - Or when somethiing on the file system breaks and needs fixing
- **Bash scripts**
  - A lot of problems can be solved using Bash scripts
- **binaries**
  - In some cases, we need to use binaries to solve the problem

- **Why building s/w packges**
  - To be able to leverage the integratio and automation cane make your administration tasks a lot easier

### Build Packages

- how to package binary packages into formats compatible with famous linux distros like
  - `APT` used by `Debian`/ `Ubuntu`,
  - `RPM` package manager used by `CentOS`/ `RedHat`
  - `snaps`
    - relatively newer standalone systems `snap` and `AppImage`

#### Debian

- Create hello-word application
- Compile the application
- Package the application
- Build a network repository

- Not covered
  - creating man pages
  - preparing our package for inclusion in official repos

- **Setting Debian server environment**
  - `apt install build-essential tree`
    - Will install all the resources that are required to compile and then package our software
- [**Debian directory**](images/debian_dir_1.png)
  - Create some directories where the **Debian package builder will look for the resources it needs**
    - `mkdir -p mycodeplace/DEBIAN`
  - This is where we will create a **control file** containing the metadata that will be associated with the package and `/usr/bin` is where we will put our binary once we create it
    - `mkdir -p mycodeplace/usr/bin`
- [**Create your project**](https://bootstrap-it.com/troubleshooting/)

    **mycode.cc (c++ program for Debian)**

    ```cpp
    #include <iostream>
    int main() {
        using namespace std;
        cout << "Welcome to mycodeplace\n";
    }
    ```

    `g++ mycode.cc -o mycode`

- [**Build debian package**](images/debian_building_mycode_package_1.png)

    **mycodeplace/DEBIAN/control file (for Debian):**

    ```bash
    Package: mycode
    Version: 1.0
    Section: custom
    Priority: optional
    Architecture: all
    Essential: no
    Installed-Size: 1024
    Maintainer: test-it.com
    Description: Proof of concept
    ```

    `cp mycode mycodeplace/usr/bin`

    `dpkg-deb --build mycodeplace`

- **Configure repository service**
  - This is needed so that remote clients can get our package and install it
  - **Install server**
    - `apt install apache2`
  - **Create repo**
    - `mkdir /var/www/html/debian`
  - **Copy our package**
    - `cp mycodeplace.deb /var/ww/html/debian/`
    - `cd /var/www/html/debian/`
  - [**Create repository package list**](images/create_repo_package_list_1.png)
    - `dpkg-scanpackages . | gzip -c9 > Packages.gz`
      - This will server as the repository's official package list

- **Using our custom repository**

- **Add server to repo list**
  - `vi /etc/apt/sources.list`
    - `deb [trusted=yes] http://10.0.3.90/debian ./`
- **Update system**
  - `apt update`
    - This will add our new software source to the system
- **Install mycode**
  - `apt install mycode`

### Creating an RPM S/W Package

  ```bash
  echo '%_topdir %(echo $HOME)/rpmbuild' > ~/.rpmmacros
  mkdir -p ~/rpmbuild/{BUILD,BUILDROOT,RPMS,SOURCES,SPECS,SRPMS}
  ```

  **helloworld-1.0/main.c:**

  ```cpp
  #include <stdio.h>
  int main (int argc, char *argv[]) {
    printf("Welcome to Pluralsight\n");
    return 0;
  }
  ```

  **helloworld-1.0/Makefile**

  ```Makefile
  DESTDIR ?=
  PREFIX ?= /usr/local

  helloworld:
    gcc main.c -o helloworld

  install: helloworld
    install -m 0755 -d $(DESTDIR)$(PREFIX)/bin
    install -m 0755 helloworld $(DESTDIR)$(PREFIX)/bin
  ```

  **helloworld.spec:**

  ```spec
  Name:           helloworld
  Version:        1.0
  Release:        1%{?dist}
  Summary:        A hello world RPM program

  License:        GPLv3+
  URL:            https://bootstrap-it.com
  Source0:        helloworld-1.0.tar.gz

  Requires(post): info
  Requires(preun): info

  %description
  helloworld from Pluralsight

  %global debug_package %{nil}

  %prep

  %setup

  %build
  make PREFIX=/usr %{?_smp_mflags}

  %install
  make PREFIX=/usr DESTDIR=%{?buildroot} install

  %clean
  rm -rf %{buildroot}

  %files
  %{_bindir}/helloworld
  ```

- **Setup package build environment**
  - [`sudo -S yum install rpm-build`](images/rpm_build_setup_1.png)
- [**Create hidden file**](images/rpm_create_hidden_file_1.png)
  - `echo '%_topdir %(echo $HOME)/rpmbuild' > ~/.rpmmacros`
    - This creates
- [**Create directory hierarchy for rpmbuild to use**](images/rpm_create_dir_hierarchy_1.png)
  - `mkdir -p ~/rpmbuild/{BUILD,BUILDROOT,RPMS,SOURCES,SPECS,SRPMS}`
- **create source file directory**
  - `mkdir helloworld-1.0`

## Tools

- Linux monitoring and administration tools
  - Cockpit
  - rpm-build
  - smartctl
  - memtester

## Study Links

- [<bootstrap-it.com/troubleshooting>](https://bootstrap-it.com/troubleshooting/)
  - Linux System Maintenance and Troubleshooting
- [Interpreting `/proc/meminfo`](https://access.redhat.com/solutions/406773)
  - Interpreting /proc/meminfo and free output for Red Hat Enterprise Linux
- Linux Performance Monitoring and Tuning
- [Disaster Recovery Reform Act](fema.gov/disaster/disaster-recovery-reform-act-2018#:~:text=These%20reforms%20acknowledge%20the%20shared,for%20the%20next%20catastrophic%20event.)
- [ISO/IEC 27031:2011
Information technology — Security techniques — Guidelines for information and communication technology readiness for business continuity](iso.org/standard/44374.html)