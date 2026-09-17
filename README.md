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

- Git
- CMake 3.16+
- Ninja
- Qt6 (Core, Gui and Widgets)
- A C++20 capable compiler

### 1) Clone the repo and initialize vcpkg

```bash
git clone https://github.com/SyrupStudio/Actinium.git
cd Actinium
git submodule update --init --recursive
```

If you cloned without `--recursive`, run the last command again.

### 2) Bootstrap vcpkg

```bash
cd vcpkg
./bootstrap-vcpkg.sh
```

On Windows PowerShell:

```powershell
cd vcpkg
./bootstrap-vcpkg.bat
```

Then set the environment variable for the current shell session:

```bash
export VCPKG_ROOT="$PWD"
```

On PowerShell:

```powershell
$env:VCPKG_ROOT = (Get-Location).Path
```

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

2. Build

```bash
  cmake -B build -G "Ninja" -DCMAKE_BUILD_TYPE=Release
  cmake --build build
```
3. Run

```bash
./build/Actinium
```

### Windows

1. Install [CMake](https://cmake.org/download/), [Ninja](https://github.com/ninja-build/ninja/releases), and [Qt6](https://www.qt.io/download-qt-installer).
2. Open a **Developer Command Prompt for VS** or ensure Qt is on `PATH`.
3. Configure:

```powershell
cmake -S . -B build -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DCMAKE_TOOLCHAIN_FILE="%VCPKG_ROOT%\scripts\buildsystems\vcpkg.cmake" -DCMAKE_PREFIX_PATH="C:\Qt\6.x\msvc2019_64"
```

4. Build:

```powershell
cmake --build build --config Release
```

5. Run:

```powershell
build\src\Actinium.exe
```

### macOS

1. Install dependencies
```bash
brew install cmake ninja qt@6
```

2. Configure with vcpkg and the macOS Qt prefix
```bash
export VCPKG_ROOT="$PWD/vcpkg"
cmake -S . -B build -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DCMAKE_TOOLCHAIN_FILE="$VCPKG_ROOT/scripts/buildsystems/vcpkg.cmake" -DCMAKE_PREFIX_PATH="$(brew --prefix qt@6)"
```

3. Build

```bash
cmake --build build
```

4. Run the app

```bash
./build/src/Actinium
```

5. Optional: bundle the app for distribution

```bash
macdeployqt build/src/Actinium
```

The repo also includes a macOS preset:

```bash
cmake --preset macos-release
cmake --build --preset macos-release
```

---

## License Acknowledgements

This project is licensed under the BSD 3-Clause License. See [LICENSE](LICENSE) for the full license text.

This project uses the following third-party components:

- Qt 6 — licensed under the GNU Lesser General Public License v3.0 (LGPLv3) or commercial Qt license terms. See https://www.qt.io/licensing/
- CMake — licensed under the BSD 3-Clause License. See https://cmake.org/licensing/
- vcpkg — licensed under the MIT License. See https://github.com/microsoft/vcpkg/blob/master/LICENSE.txt
- Lua / LuaJIT — licensed under the MIT License / Lua License terms. See https://opensource.org/licenses/MIT and https://luajit.org/license.html

Where applicable, the licensing terms for downstream dependencies are included or referenced by their upstream projects. Please review the relevant licenses before redistribution or commercial use.

---
