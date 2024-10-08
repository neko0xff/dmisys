加入本函式庫至自己的專案內
===

[回開發說明文件主頁](README.md)


## 加入依賴
- 使用Github倉庫源
    1. 加入所需的相依
        * 手動: 在專案內的`Cargo.toml`進行設置
            ```toml=
            [dependencies]
            dmisys = { git = "https://github.com/neko0xff/dmisys.git", branch = "main" }
           ```
        * 命令列: `cargo add dmisys --git https://github.com/neko0xff/dmisys.git --branch main`
- 使用`carte.io`官方套件庫
  *  命令列: `$ cargo add dmisys`

## 在程式內使用剛加入依賴的套件
1. 在程式碼內調用函式庫
     ```rust=
    use dmisys::*;

    fn main(){
        println!("OS: {}",dmisys::os::read_osname());
    }
     ```
2. 開始編譯！: `cargo run`