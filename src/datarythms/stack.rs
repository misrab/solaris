pub struct Stack<T> {
  data: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Self {
    Stack{
      data: Vec::new(),
    }
  }

  fn pop(&mut self) -> Option<T> {
    self.data.pop()
  }
  
  fn push(&mut self, v: T) {
    self.data.push(v);
  }
}


#[test]
fn test_new() {
  let _ = Stack::<u8>::new();
}

#[test]
fn test_pop_empty() {
  let mut q = Stack::<u8>::new();
  let result = q.pop();
  assert_eq!(None, result);
}

#[test]
fn test_push_pops() {
  let mut s = Stack::<u8>::new();
  s.push(1);
  s.push(2);
  assert_eq!(s.pop(), Some(2));
  assert_eq!(s.pop(), Some(1));
  assert_eq!(s.pop(), None);
}

