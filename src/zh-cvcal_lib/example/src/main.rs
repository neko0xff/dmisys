use zh_cvcal_lib::{*};

fn main() {
    let result1 = year::ad_cv_roc(2024);
    let result2= year::roc_cv_ad(113);
    let num1:u8 = true as u8;
    let num2:u8 = false as u8;
    let result3 = num::usize_to_bool((num1+num2).into());

    println!("ROC: {:?}",  result1);
    println!("AD: {:?}",  result2);
    println!("Bool: {:?}",result3);
}
