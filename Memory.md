# Memory

## Measuring Memory Usage

- <http://locklessinc.com/articles/memory_usage/>
  - using C code explains how to measure the measure memory usage of a process
    - "/proc/pid/status"
      - uses "VmRSS"

        ```bash
        VmPeak: Peak virtual memory usage
        VmSize: Current virtual memory usage
        VmLck:  Current mlocked memory
        VmHWM:  Peak resident set size
        VmRSS:  Resident set size
        VmData: Size of "data" segment
        VmStk:  Size of stack
        VmExe:  Size of "text" segment
        VmLib:  Shared library usage
        VmPTE:  Pagetable entries size
        VmSwap: Swap space used
        ```

## GNU Statustics on malloc

- <http://www.gnu.org/software/libc/manual/html_node/Statistics-of-Malloc.html>
  - Most allocators provide these stats somewhere. For example, GNU libc has mallinfo.
    - <https://stackoverflow.com/questions/36185783/linux-proc-self-statm-is-it-trustable>
