pub struct Queue<T> {
  data: Vec<T>,
}

// This is implemented with 
// slow push, fast pop
impl<T> Queue<T> {
  fn new() -> Self {
    Queue{
      data: Vec::new(),
    }
  }

  // currently O(1)
  fn pop(&mut self) -> Option<T> {
    self.data.pop()
  }
 
  // currently O(n): shift all elems right
  fn push(&mut self, v: T) {
    self.data.insert(0, v);
  }
}


#[test]
fn test_new() {
  let _ = Queue::<u8>::new();
}

#[test]
fn test_pop_empty() {
  let mut q = Queue::<u8>::new();
  let result = q.pop();
  assert_eq!(None, result);
}

#[test]
fn test_push_pops() {
  let mut q = Queue::<u8>::new();
  q.push(1);
  q.push(2);
  assert_eq!(q.pop(), Some(1));
  assert_eq!(q.pop(), Some(2));
  assert_eq!(q.pop(), None);
}

