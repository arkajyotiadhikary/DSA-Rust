use std::collections::HashMap;
// a func takes string as input
// take a pointer to the first character.
// check other checter from the string .
// if found same chracter break and move the poitner to the next cheracter.
pub fn first_non_repeating_chracter_using_pointer(s:&str)->char{
  let mut chars = s.chars();
  let mut pointer = chars.next().unwrap();
  while let Some(ch) = chars.next(){
    if ch == pointer{
      pointer = chars.next().unwrap();
    }
  }
  pointer
}

pub fn first_non_repeating_chracter_using_hashmap(s:&str)->char{
  let mut map = HashMap::new();
  for ch in s.chars(){
    *map.entry(ch).or_insert(0) += 1;
  }
  for ch in s.chars(){
    if map[&ch] == 1{
      return ch;
    }
  }
  '\0'

}
