 //fn main() {
//    let mut s1 = String::from("abc");
//    do_stuff(&mut s1);
//    println!("{}", s1);
//    fn do_stuff(s: &mut String){
//        //s.insert_str(0, "hi, ");
//        *s = String::from("Replacement");
//    }
//}
// fn main(){
     
//     let s1 = String::from("abc");
//     let _s2 = s1.clone();
//     println!("s1 = {}, s2 = {}",s1, _s2);
//     
//}

fn main(){
    let s = String::from("hello");
    takes_ownships(s);    

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownships(some_strings: String) {
    println!("{}", some_strings);

}


fn makes_copy(some_integer: i32) {
    println!("{}", some_integer + 100);

}

