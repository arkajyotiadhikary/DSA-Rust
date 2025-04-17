// * NOTE: For queue we need dequeue lib from std.
use std::collections::VecDeque;

// stack using queue

// we can have to two kinds of stack . LIFO [last in first out] and FIFO [first in first out]
pub struct Stack{
  queue1:VecDeque<i32>, // FIFO
  queue2:VecDeque<i32>, // LIFO
}


impl Stack{
  // constructor
  pub fn new() -> Self {
    Stack{
      queue1:VecDeque::new(),
      queue2:VecDeque::new(),
    }
  }

  // push
  pub fn push(&mut self,x:i32){
    self.queue2.push_back(x);
    while let Some(elem) = self.queue1.pop_front(){
      self.queue2.push_back(elem);
    }
    std::mem::swap(&mut self.queue1,&mut self.queue2);
  }
    // pop
   pub fn pop(&mut self) -> Option<i32> {
      self.queue1.pop_front()
    }

    // top
   pub fn top(&self)->Option<i32>{
      self.queue1.front().copied()
    }

    //empty
    pub fn empty(&self)->bool{
      self.queue1.is_empty()
    }
}
