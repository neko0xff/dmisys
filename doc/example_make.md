建置範例程式
===

[回開發說明文件主頁](README.md)

## 使用內部的makefile建置
1. 到範例的專案目錄: `cd example`
2. 輸入`make`
     * 預設動作
        1. 輸出所使用的工具的版本: `make version` 
        2. 清除先前建置的二進制檔案: `make clean`
        3. 開始建置: `make run_release`

## 執行後結果
```zsh
# user @ Host-02 in ~/文件/GitHub/systools_rs/example on git:main x [10:52:48] C:2
$ ./target/release/example --help
Use dimsys libary, make a CLI Side Project

Usage: example [OPTIONS]

Options:
  -o, --output <OUTPUT>
          Choose need output a information data! (You can output: cpu,gpu,disk,memory,network,power,system)
          
          [default: all]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

# user @ Host-02 in ~/文件/GitHub/systools_rs/example on git:main x [10:52:51] 
$ ./target/release/example --output system
System Information
=================
OS:             Linux  Arch Linux
Distro:         Arch Linux
Host Name:      Host-02
Kernel:         6.11.1-zen1-1-zen
Vendor:         Micro-Star International Co., Ltd.
Board:          MS-168C
Model:          CR643
Uptime:         0 days, 5 hours, 17 minutes
Unix time:      20001 days, 21 hours, 48 minutes
IO:             Write = 15181 MB / Read = 5099 MB

BIOS Information
=================
Vendor:         American Megatrends Inc.
Release:        4.6
Version:        E168CIMS.30N
Date:           04/29/2011
```