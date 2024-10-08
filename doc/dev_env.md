開發環境
===

[回開發說明文件主頁](README.md)

## 裝置1: MS-168C(MSI CR643)
```zsh
# user @ Host-02 in ~/文件/GitHub/systools_rs/example on git:main x [14:01:22] 
$ sudo ./target/release/example --output system
System Information
=================
OS:             Linux  Arch Linux
Distro:         Arch Linux
Host Name:      Host-02
Kernel:         6.11.1-zen1-1-zen
Vendor:         Micro-Star International Co., Ltd.
Board:          MS-168C
Model:          CR643
Uptime:         0 days, 8 hours, 12 minutes
Unix time:      20001 days, 21 hours, 48 minutes
IO:             Write = 17206 MB / Read = 6784 MB

BIOS Information
=================
Vendor:         American Megatrends Inc.
Release:        4.6
Version:        E168CIMS.30N
Date:           04/29/2011

# user @ Host-02 in ~/文件/GitHub/systools_rs/example on git:main x [14:01:41] 
$ sudo ./target/release/example --output cpu   

CPU Information
================
CPU Model:           "Intel(R) Core(TM) i5-2450M CPU @ 2.50GHz"
CPU Frequency:       1.90 GHz
CPU Core:            2
CPU Threads:         4
CPU Arch:            x86_64
CPU Load Avg:        1.93%

# user @ Host-02 in ~/文件/GitHub/systools_rs/example on git:main x [14:03:02] 
$ sudo ./target/release/example --output power

Power Information
==================
Autosuspend Delay:        Unknown ms
Control:                  auto
Runtime Status:           unsupported
Runtime Active Time:      0
Runtime Suspended Time:   0

Power Supply Information
=========================
Find Device:
ADP : 1, BAT: 1

ADP List
+--------+------+-------+--------+
| number | name | type_ | online |
+--------+------+-------+--------+
| 1      | ADP1 | Mains | Yes    |
+--------+------+-------+--------+

BAT Info
+--------+------+---------+--------------+---------+------------+----------+-------------+---------+--------------+-----------+
| number | name | type_   | status       | present | technology | capacity | capacity_lv | model   | manufacturer | serialnum |
+--------+------+---------+--------------+---------+------------+----------+-------------+---------+--------------+-----------+
| 1      | BAT1 | Battery | Not charging | Yes     | Unknown    | 98       | Normal      | MS-1727 | MSI Corp.    |           |
+--------+------+---------+--------------+---------+------------+----------+-------------+---------+--------------+-----------+

BAT Electron Info
+--------+------+----------+----------+------------+--------------------+-------------+------------+
| number | name | volt_min | volt_now | charge_now | charge_full_design | current_now | cyclecount |
+--------+------+----------+----------+------------+--------------------+-------------+------------+
| 1      | BAT1 | 11.1     | 12.435   | 3404       | 4400               | 0           | 0          |
+--------+------+----------+----------+------------+--------------------+-------------+------------+

BAT Health Info
+--------+------+-----------+------------+
| number | name | life_time | percentage |
+--------+------+-----------+------------+
| 1      | BAT1 | 0.00      | 77.36      |
+--------+------+-----------+------------+

```

## 裝置2: F2A78M-DS2
```zsh
➜  example git:(main) ✗ sudo ./target/release/example --output system
System Information
=================
OS:             Linux 24.05 NixOS
Distro:         NixOS
Host Name:      Host01
Kernel:         6.9.8-xanmod1
Vendor:         Gigabyte Technology Co., Ltd.
Board:          F2A78M-DS2
Model:          To be filled by O.E.M.
Uptime:         0 days, 1 hours, 40 minutes
IO:             Write = 2504 MB / Read = 3658 MB

BIOS Information
=================
Vendor:         American Megatrends Inc.
Release:        4.6
Version:        F6e
Date:           05/05/2015

➜  example git:(main) ✗ sudo ./target/release/example --output cpu

CPU Information
================
CPU Model:           "AMD A8-7600 Radeon R7, 10 Compute Cores 4C+6G"
CPU Frequency:       3.09 GHz
CPU Core:            4
CPU Threads:         4
CPU Arch:            x86_64
CPU Load Avg:        3.22%

➜  example git:(main) ✗ sudo ./target/release/example --output power

Power Information
==================
Autosuspend Delay:        Unknown ms
Control:                  auto
Runtime Status:           unsupported
Runtime Active Time:      0
Runtime Suspended Time:   0

```