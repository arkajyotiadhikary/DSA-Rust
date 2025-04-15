// * we can merge two array in few different ways
// below is the way where we are merging two arrays while making sure they are also sorted.


pub fn merge_two_arrays(arr1:&[i32],arr2:&[i32])->Vec<i32>{

  // * First we have to create a a vector with_capacity();
  let mut merged = Vec::with_capacity(arr1.len()+arr2.len());

  let(mut i,mut j) = (0,0);

  // * if we
  while i < arr1.len() && j < arr2.len(){
    if arr1[i] <= arr2[j]
    {
      merged.push(arr1[i]);
      i += 1;
    }
    else{
      merged.push(arr2[j]);
      j += 1;
    }
  }

  // * after sorting we might left with few extra elements
  // * also below is a way where you just want to merge two array without any sorting
  if i < arr1.len(){
    merged.extend_from_slice(&arr1[i..]);
  }
  if j < arr2.len(){
    merged.extend_from_slice(&arr2[j..]);
  }

  // return
  merged

}
