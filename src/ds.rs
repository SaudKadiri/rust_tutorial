use std::collections::VecDeque;

struct Stack<T> {
    stack: Vec<T>
}
impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    pub fn length(&self) -> usize {
        self.stack.len()
    }
    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
    pub fn push(&mut self, val: T) {
        self.stack.push(val);
    }
    pub fn pop(&mut self) {
        self.stack.pop();
    }
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }
}

struct Queue<T> {
    queue: VecDeque<T>
}
impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: VecDeque::new() }
    }
    pub fn length(&self) -> usize {
        self.queue.len()
    }
    pub fn front(&self) -> Option<&T> {
        self.queue.front()
    }
    pub fn rear(&self) -> Option<&T> {
        self.queue.back()
    }
    pub fn push(&mut self, val: T) {
        self.queue.push_back(val);
    }
    pub fn pop(&mut self) {
        self.queue.pop_front();
    }
    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }
}

pub fn run() {
    let mut stk: Stack<i32> = Stack::new();
    println!("{:?}", (stk.peek(), stk.length(), stk.is_empty()));
    stk.push(5);
    stk.push(10);
    println!("{:?}", (stk.peek(), stk.length(), stk.is_empty()));
    stk.pop();
    println!("{:?}", (stk.peek(), stk.length(), stk.is_empty()));
    
    println!("{}", "=".repeat(50));
    
    let mut queue: Queue<i32> = Queue::new();
    println!("{:?}", (queue.front(), queue.rear(), queue.length(), queue.is_empty()));
    queue.push(5);
    println!("{:?}", (queue.front(), queue.rear(), queue.length(), queue.is_empty()));
    queue.pop();
    println!("{:?}", (queue.front(), queue.rear(), queue.length(), queue.is_empty()));
}