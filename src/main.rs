mod reverse_array;
mod palindrom;
mod merge_two_arrays;
mod first_non_repeating_character;
mod stack;

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

   let non_repeating_char = first_non_repeating_character::first_non_repeating_chracter_using_pointer("aabbc");
   println!("{}",non_repeating_char);

   let non_repeating_char = first_non_repeating_character::first_non_repeating_chracter_using_hashmap("aabbc");
   println!("{}",non_repeating_char);

   let mut stack = stack::Stack::new();
    stack.push(1);
    stack.push(2);
    println!("{:?}", stack.top());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.empty());

}
