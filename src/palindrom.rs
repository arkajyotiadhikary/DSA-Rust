// take string as input
// use two pointer s untill you cross or meet each other

pub fn palindrom(str:&str)->bool{
      let mut left:usize = 0;
      let mut right:usize = str.len() -1;

      while left<right{
            if str.chars().nth(left).unwrap() != str.chars().nth(right).unwrap(){
                  return false;
            }
            left +=1;
            right -=1;
      }
      true
}
