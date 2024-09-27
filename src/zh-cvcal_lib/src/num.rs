// 無符號整數轉布林
pub fn usize_to_bool(n: usize) -> bool{
    let mut out = false;
    
    if n == 0 {
        out = false;
    }else if n == 1 {
        out = true;
    }

    out 
}

// 布林轉無符號整數
pub fn bool_to_usize(n: bool) -> usize{
    let mut out:usize = 0;

    if n == false {
        out = 0;
    }else if  n== true {
        out = 1;
    }

    out
}