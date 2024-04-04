#![allow(unused)]

fn main() {
    //types();
    //tuples();
    //array();
    // println!("{}",is_even(10));
    // println!("{}",is_even(11));
    // println!("{}",is_even(215));
    //ifelse(104);
    //_loop();
    //_while();
    _for();

}


fn types (){
    let int_64_var: i64 = 12;
    println!("{}", int_64_var);

    let (n1, n2, n3): (i16, u16, i128) = (0, 0, 0);

    let myfloat = 3.1416;
    println!("{}", myfloat);

    let is_alpha = true;
    let is_char: bool = 'p' >= 'a' && 'p' <= 'z';
    println!("{}", is_char); 

}

fn tuples() {
    let my_tuple : (i8, i16, f32) = (i8::MAX, i16::MAX, 200.369);
    println!("{:?}", my_tuple);

    let second_element = my_tuple.1;
    println!("second element: {}", second_element);

    //destruct

    let (x, y, z) = my_tuple;
    println!("X: {}, y: {}, z: {}", x, y, z);
    
}

fn array() {
    let array: [i8; 5] = [1,2,3,4,5];
    println!("Array: {:?}", array);
    println!("3rd element: {}", array[2]);

    //destruct
    let [a, b, c, d, e] = array;

}

fn is_even(number: i32) -> bool {
    return number % 2 == 0;
}

fn ifelse(number: i32)  {
    if is_even(number) {
        println!("OkOK");
    }
    else {
        println!("Not OKOK");
    }

    let num = if(is_even(number)) {
        1000
    }
    else {
        0
    };

    println!("Eda result: {}", num);
}

fn _loop(){
    let mut i = 0;
    loop {
        println!("Looping {} times.", i);
        i += 1;

        if i > 100 {
            break;
        }
    }
}

fn _while() {
    let mut i = 0;
    while i < 20 {
        print!("{} ", i);
        i += 2;
    }
}

fn _for(){
    let a: [i8; 5] = [1, 2, 3, 4, 5];
    for value in a.iter() {
        print!("{} ", value);
    }
    println!()

    // let tup = (1, 2, 4, 5, 89);
    // for value_of_index in [1, 2, 3, 4, 5].iter() {
    //     print!("{} ", tup.value_of_index);
    // }
}