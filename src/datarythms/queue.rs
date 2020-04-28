pub struct Queue<T> {
  data: Vec<T>,
}

impl<T> Queue<T> {
  fn new() -> Self {
    Queue{
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
  q.push(3);
  assert_eq!(q.pop(), Some(3));
  assert_eq!(q.pop(), None);
}

