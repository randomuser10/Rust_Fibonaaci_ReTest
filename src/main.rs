fn main() {
    gen_fibonacci(10);
}

fn gen_fibonacci(n: u32) -> (){
    let mut _num1 = 1;
    let mut _num2 = 1;
    let mut _new_num = 0;
    let mut i = 0;

    while i <= n {
        if i <= 1{
            println!("{i}");
        } else if i == 2{
            _new_num = 1 + 0;
            println!("{_new_num}");
            _new_num = 0;
        }
          else {
            _new_num = _num1 + _num2;
            println!("{_new_num}");
            _num1 = _num2;
            _num2 = _new_num;
          }
          i += 1;


    }
}