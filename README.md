<div align="center">
    <img src="logo.png" alt="OpenMover logo" width="128">
    
    # Actinium

    **⚠️ Actinium is currently in development.**

[![Release](https://img.shields.io/github/v/release/SyrupStudio/Actinium?include_prereleases)](https://github.com/SyrupStudio/Actinium/releases)
[![License: BSD 3-Clause](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![Issues](https://img.shields.io/github/issues/SyrupStudio/Actinium)](https://github.com/SyrupStudio/Actinium/issues)
[![Last Commit](https://img.shields.io/github/last-commit/SyrupStudio/Actinium)](https://github.com/SyrupStudio/Actinium/commits/main)
[![C++](https://img.shields.io/badge/C%2B%2B-20-blue.svg)](https://en.cppreference.com/w/cpp/20)
[![Qt](https://img.shields.io/badge/Qt-6-41CD52.svg?logo=qt)](https://www.qt.io/)
[![Platforms](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)](#compiling)
</div>

**This readme is currently incomplete**

## About
Actinium is a game engine made in C++ and uses Lua as the language to script games.

---

## Compiling

### Prerequisites (all platforms)
- CMake 3.16+
- Ninja
- Qt6 (Core, Gui and Widgets)
-  A C++20 capable compile

### Linux (Debian, Ubuntu based distros)

```bash
    sudo apt update
    sudo apt install -y build-essential cmake ninja-build qt6-base-dev
```

### Linux (Fedora, RHEL based distros)

```bash
    sudo dnf install -y gcc-c++ cmake ninja-build qt6-qtbase-devel
```

### Linux (Arch based distros)
```bash
    sudo pacman -S --needed base-devel cmake ninja qt6-base
```

Once you have downloaded the dependencies to build

1. Clone the repo
```bash
    git clone https://github.com/SyrupStudio/Actinium.git
    cd Actinium
```

2.  Build
```bash
  cmake -B build -G "Ninja" -DCMAKE_BUILD_TYPE=Release
  cmake --build build
```
3. Run
```bash
./build/Actinium
```

### Window (There might problems i dont use windows)

1. Install [CMake](https://cmake.org/download/), [Ninja](https://github.com/ninja-build/ninja/releases), and [Qt6](https://www.qt.io/download-qt-installer) (make sure the MSVC or MinGW component matching your compiler is selected).
2. Open a **Developer Command Prompt for VS** (or ensure your compiler and Qt's `bin` directory are on `PATH`).
3.  Build
```powershell
cmake -B build -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DCMAKE_PREFIX_PATH="C:\Qt\6.7.0\msvc2019_64"
cmake --build build --config Release
```
4. Deploy Qt runtime DLLs
```powershell
windeployqt --release build\Actinium.exe
```
5. Run
```powershell
build\Actinium.exe
```

### MacOS (Might be problems here I dont own a mac)

1. Download dependencies
```bash
brew install cmake ninja qt@6
```

2. Build
```bash
cmake -B build -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DCMAKE_PREFIX_PATH="$(brew --prefix qt@6)"
cmake --build build
```

3. Bundle the Qt runtime into the `.app`
```bash
macdeployqt build/Actinium.app
```

5. Run
```bash
open build/Actinium.app
```

---






