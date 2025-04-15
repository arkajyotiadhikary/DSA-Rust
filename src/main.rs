mod reverse_array;
mod palindrom;
mod merge_two_arrays;

fn main() {
    let arr:[i32;5] = [1,2,3,4,5];
    let arr1:&[i32] = &[1,2];
    let arr2:&[i32] = &[1,2];

    reverse_array::reverse_array_using_build_in_function(arr);
    reverse_array::reverse_array_using_two_pointer(arr);
   if palindrom::palindrom("raceca"){
      println!("it is a palindrom");
   }else{
      println!("it is not a palindrom");
   }
   let merged_arr = merge_two_arrays::merge_two_arrays(arr1, arr2);
   println!("{:?}" ,merged_arr);
}
