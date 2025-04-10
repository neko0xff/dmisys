# dmisys

一個以 Rust 撰寫的輕量級函式庫，可用來讀取 Linux 系統的設定與環境資訊。

[![Crates.io](https://img.shields.io/crates/v/dmisys.svg)](https://crates.io/crates/dmisys)
[![Docs.rs](https://docs.rs/dmisys/badge.svg)](https://docs.rs/dmisys)

---

## README Version

- 🌐 [繁體中文(zh-tw)](./README_zh-tw.md)
- 🌐 [English](./README.md)

---

## 維護者

- **neko_0xff**（[@neko0xff](https://github.com/neko0xff)）

---

## 想法

本專案旨在使用 Rust 語言讀取 Linux 上的各種系統資訊，並嘗試將這些功能模組化、函式化，方便其他應用程式進行呼叫與整合。

---

## 支援平台

- **Linux**（目前僅支援 Linux）

---

## 功能

- **系統資訊讀取**  : 調用系統相關的資訊，例如記憶體與磁碟使用狀況等。

- **HTTP 請求工具**  : 提供 HTTP 請求的簡易工具，例如：取得本機的公開 IP 位址。

- **單位轉換**  : 常見單位的轉換工具，例如 byte → MB/GB 等。

---

## 文件

- 📂 [範例程式](./example/)
- 📖 [使用說明](./doc/README.md)

---

## 授權

本專案採用 [GPL-3.0 授權條款](./LICENSE)。
