use super::*;

#[test]
fn check_stuff(){
    let result = sum(2, 2);
    assert_eq!(result, 4);
}
#[test]
fn check_stuff2(){
    let result = diff(2, 2);
    assert_eq!(result, 0);
}
#[test]
fn check_stuff3(){
    let result = pro(2, 2);
    assert_eq!(result, 4);
}
#[test]
fn check_stuff4(){
    let result = quo(2.0, 2.0);
    assert_eq!(result, 1.0);
}
#[test]
fn check_stuff5(){
    let result = rem(2.0, 2.0);
    assert_eq!(result, 0.0);
}
/*  #[test]
fn check_stuff6(){
    // sum
    println!("sum : {}", sum(234, 2)); // 'sum : 236'
    println!("sum : {}", sum(1, 255)); // 'ERROR: attempt to add with overflow'
    // diff
    println!("diff : {}", diff(234, 2)); // 'diff : 232'
    println!("diff : {}", diff(-32768, 32766)); // 'ERROR: attempt to subtract with overflow'
    // product
    println!("pro : {}", pro(23, 2)); // 'pro : 46'
    println!("pro : {}", pro(-128, 2)); // 'ERROR: attempt to multiply with overflow'
    // quotient
    println!("quo : {}", quo(22.0, 2.0));// 'quo : 11'
    println!("quo : {}", quo(-128.23, 2.0));// 'quo : -64.115'
    // remainder
    println!("rem : {}", rem(22.0, 2.0));// 'rem : 0'
    println!("rem : {}", rem(-128.23, 2.0));// 'rem : -0.22999573'
} 
  */