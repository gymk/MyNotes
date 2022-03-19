# `vi` editor

- [`vi` editor](#vi-editor)
  - [Common `vi` Commands](#common-vi-commands)
    - [File Operations](#file-operations)
    - [Navigation](#navigation)
    - [Editing Text](#editing-text)
    - [`vimtutor`](#vimtutor)
  - [Other text editors](#other-text-editors)
  - [Study Links](#study-links)

- Installation
- Understanding command execution in the bash shell
- Managing files and file comparison
- **Using the vi text editor**
- Configuring and managing file permissions
- Advanced shell topics and searching for text in files
- Finding help

- Everything in Linux is **file**
  - Efficient and effective command of a text editor is a crucial skill for any Linux System Administrator

- What is VI/VIM
  - vi
    - Text editor, very simple
  - vim
    - vi iMproved
      - syntax highlighting
      - split screen
      - differencing files
  - **Why Letters for navigation**
    - [Old Keyboard](images/old_keyboard_1.png)
      - No arrow keys for moving the cursor
      - No page up/down keys for navigating in a file
    - Hence, keybaord letters are used for navigation
      - `h`,`j`,`k`,`l` for moving cursors
      - `CTRL + f`, `CTRL + b` for page up and page down
    - [Rect Keyboard](images/recent_keyboard_1.png)
      - arrow keys and page up/down keys are available, and it can be used
- VI's Modes
  - How to interact with it and in which modes
  - In each mode, key sequence is interpreted differently
  - **Command Mode**
    - Instructions to VI to perform functions
      - *save, copy/paste, quit, search, ...*
    - **Insert Mode**
      - Actual text editing
- Creating, opening and saving a file
  - How to work with files
- basic navigation commands
- Editing a text file

## Common `vi` Commands

### File Operations

- Mode Command Mode
  - Save
    - save - `:w`
    - Save to a new file - `:w filename`
  - Quitting
    - `:q` - quit if no changes
    - `q!` - quit without saving
    - `wq` - save and quit

### Navigation

- Mode Command Mode
  - **Moving cursor**
    - up/down/left/right
    - `k` / `j` / `h` / `l`
  - **Moving between words**
    - `w` - Move right one word
    - 'b' - Move left one word
  - **Start of file**
    - `[[` to move to the start of the file
    - `:1` to move to the first line of the file
  - **End of file**
    - `]]` to move to the end of the file
    - `SHIFT + g` to go to the end of the file
  - **Page Up/Down**
    - `CTRL + F` to move one page down
    - `CTRL + B` to move one page up
  - **Move to start of line**
    - `0` (zero)
  - **Move to end of line**
    - `$`
  - **Search for text forward**
    - `/searchstring` it will highlight all the matching search string
    - `:nohl` to remove the highlighting
  - **Find and Replace**
  - `:s/s1/s2` `s1` stirng to find `s2` to be replaced

### Editing Text

- There is a lot of different ways to edit the text

- **Inserting**
  - Mode Command Mode
    - `a` - append current position
    - `A` - append end of line
    - `i` - insert current position
    - `I` - insert beginning of line
    - `R` - replace current position
    - `o` - start new line, below current position
    - `O` - start new line, above current position
- **Copy and Past**
  - Mode Command Mode
    - `yy` - copy current line to buffer
    - `p` - paste line in buffer below current line
    - `n p` - `n` is a number, past `n` time
- **Undo**
  - Mode Command Mode
    - `u` - undo the last edit
- **Deleting Lines**
  - `dd` - delete one line
  - `n dd` - delete `n` lines of text
- **Deleting Characters**
  - `x` - delete character to the right
  - `X` - delete character to the left
- **Editing in parallel**
  - `vi -O largeFile1 largeFile2`
    - Use `CTRL + w` to switch between the files
      - `CTRL + w + l` to move to the right file
      - `CTRL + w + h` to move to the left file

### `vimtutor`

- Normally included in most Linux distros

## Other text editors

- `nano`
  - UI is more friendly than `vi`
    - As it has menu based system on the bottom of the screen
- `emacs`
  - Very customizable editor
  - It is modeless
    - Syntax highlighting and advanced macro features

## Study Links

- <www.centinosystems.com>
- Understanding and Using Essential Tools for Enterprise Linux 7
  - Using the `vi` Text Editor
