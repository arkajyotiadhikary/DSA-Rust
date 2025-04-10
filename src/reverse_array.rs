// first lets try with in build rust function that can reverse an array
pub fn reverse_array_using_build_in_function(arr: [i32;5]) {
    // Create a mutable array of book titles
    
    // Print the original array
    println!("Original order: {:?}", arr);
    
    // Reverse the array
    let mut arr = arr;
    arr.reverse();      
    
    // Print the reversed array
    println!("Reversed order: {:?}", arr);
}

pub fn reverse_array_using_two_pointer(mut arr:[i32;5]){
      let mut left =0;
      let mut right = arr.len()-1;

      while left <right{
            let temp = arr[left];
            arr[left] = arr[right];
            arr[right] = temp;
            left +=1;
            right -=1;  
      }

      println!("Reversed order: {:?}", arr);
}