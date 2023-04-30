
pub struct Queue<T> {
    pub queue: Vec<T>
}

impl <T> Queue<T> {
    pub fn new() -> Self {
        return Queue {queue: Vec::new()}
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item);
    }

    pub fn dequeue(&mut self) {
        self.queue.remove(0);
    }

    pub fn length(&self) -> usize {
        return self.queue.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.queue.is_empty();
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        return self.queue.get(index);
    }

    pub fn update(&mut self, index: usize, value: T) {
       self.queue[index] = value;
    }

}

// pub struct QueueIter<T> {
//     queue: Queue<T>,
//     idx: usize,
// }

// impl <T>Iterator for QueueIter<T>{
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item>  {
//         let idx = self.idx;
//         self.idx += 1;
//         return std::option::Option::Some(self.queue.get(idx))
//     }
// }

// impl <T>IntoIterator for Queue<T> {
//     type IntoIter = QueueIter<T>;
//     type Item =T;
//     fn into_iter(self) -> Self::IntoIter {
//         return QueueIter {
//             queue: self,
//             idx: 0,
//         }
//     }
// }