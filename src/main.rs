mod reverse_array;
mod palindrom;

fn main() {
    let arr:[i32;5] = [1,2,3,4,5];

    reverse_array::reverse_array_using_build_in_function(arr);
    reverse_array::reverse_array_using_two_pointer(arr);
   if palindrom::palindrom("raceca"){
      println!("it is a palindrom");
   }else{
      println!("it is not a palindrom");
   }
}
