// 西元轉民國
pub fn ad_cv_roc(year: usize) -> usize{
    let ad = 1911;
    let out = year - ad;

    out
}

// 民國轉西元
pub fn roc_cv_ad(year:usize) -> usize{
    let ad  = 1911;
    let out = year + ad;

    out
}