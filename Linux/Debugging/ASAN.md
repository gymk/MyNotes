# Address Sanitizer

- Supported by both `GCC` and `clang` compilers

- What Address Sanitizer does
  - It checks for various types of bugs which related to memory
    - Memory Lekas
    - Accessing memory locations that you shouldn't be accessing
      - Memory that you have deallocated
      - ...

- From <https://github.com/mdadams/cppbook_companion/blob/master/miscellany/buggy/app.cpp>

    ```c++
    #include <algorithm>
    #include <iostream>
    #include <vector>

    int main() {
        std::vector<int> v{ 1, 2, 3 };
        auto two = std::find(v.begin(), v.end(), 2);
        for(int i = 0; i < 4096; i++) {
            v.push_back(i);
        }
        std::cout << *two << '\n';
    }
    ```

    ```cmake
    # Specify minimum required version of CMake.
    cmake_minimum_required(VERSION 3.1 FATAL_ERROR)

    # Specify project and identify languages used.
    project(app LANGUAGES CXX)

    # Enable verbose makefiles.
    set(CMAKE_VERBOSE_MAKEFILE true)

    # Add Flags
    # Configure for C++11
    set(CMAKE_CXX_FLAGS "-std=c++11")
    # Debug Level
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g3")
    # Enable Address Sanitizer (ASan) for GCC/Clang
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fsanitize=address")
    set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -fsanitize=address")

    # Add program target called app.
    add_executable(app app.cpp)
    ```

- `cmake -H. -Btmp_cmake -DCMKAE_BUILD_TYPE=Debug`
  - Configure for a debug build
- `cmake --build tmp_cmake`
  - Build the app
- `tmp_cmake/app`

## Study Links

- <https://github.com/mdadams/cppbook_companion.git>
  - Has examples
    - cppbook_companion/miscellany/buggy/
