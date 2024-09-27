 /*呼叫己寫好的函式庫 */
 extern crate zh_cvcal_lib;
 use zh_cvcal_lib::{*};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cv_rocyear(){
        let result = year::ad_cv_roc(2024);
        assert_eq!(result,113);
    }

    #[test]
    fn cv_adyear(){
        let result = year::roc_cv_ad(113);
        assert_eq!(result,2024);
    }

    #[test]
    fn cv_bool1(){
        let result = num::usize_to_bool(1);
        assert_eq!(result,true);
    }

    #[test]
    fn cv_bool2(){
        let result = num::usize_to_bool(0);
        assert_eq!(result,false);
    }
}