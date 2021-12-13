use std::fmt::Display;

type Node<T> = Option<Box<LinkNode<T>>>;
#[derive(Debug)]
pub struct LinkNode<T> {
    val: T,
    next: Node<T>,
}

#[derive(Debug)]
pub struct Link<T> {
    pub head: Node<T>,
    pub size: usize,
}

pub trait LinkNodeMethod<T> {
    fn new(val: T, next: Node<T>) -> LinkNode<T> {
        LinkNode { val, next }
    }
}

pub trait LinkMethod<T> {
    fn new() -> Link<T> {
        Link {
            head: None,
            size: 0,
        }
    }

    fn insert(self, index: usize, val: T) -> Link<T>;

    fn remove(self, index: usize) -> Link<T>;

    fn update(self, index: usize, new_val: T) -> Link<T>;

    fn show(self) -> Link<T>;
}

impl<T> LinkNodeMethod<T> for LinkNode<T> {}

impl<T> LinkNode<T> {
    fn to_box(self) -> Box<LinkNode<T>> {
        Box::new(self)
    }

    fn to_option(self) -> Node<T> {
        Some(self.to_box())
    }

    pub fn get_last(&mut self) -> &mut LinkNode<T> {
        if self.next.is_none() {
            self
        } else {
            self.next.as_mut().unwrap().get_last()
        }
    }

    /* 获取当前节点之后的第index个节点 */
    pub fn get<'a>(&'a mut self, index: usize) -> Option<&'a mut Self> {
        if index == 0 {
            return Some(self);
        }
        if self.next.is_none() {
            None
        } else {
            self.next.as_mut().unwrap().as_mut().get(index - 1)
        }
    }
}

impl<T> Link<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    fn get_last(&mut self) -> Option<&mut LinkNode<T>> {
        if self.head.is_none() {
            None
        } else {
            Some(self.head.as_mut().unwrap().get_last())
        }
    }

    pub fn push(&mut self, val: T) -> () {
        let node = Some(Box::new(LinkNode::new(val, None)));
        if self.head.is_none() {
            self.head = node;
        } else {
            self.head.as_mut().unwrap().get_last().next = node;
        }
        self.size += 1;
    }
}

impl<T: Copy + Display> LinkMethod<T> for Link<T> {
    fn new() -> Link<T> {
        Link {
            head: None,
            size: 0,
        }
    }

    fn insert(mut self, index: usize, val: T) -> Link<T> {
        if index > self.size {
            panic!("index: {} 已越界，链表元素长度为size: {}", index, self.size);
        }
        let mut new_node = LinkNode::new(val, None);
        if index == 0 {
            if self.head.is_none() {
                self.head = Some(Box::new(new_node));
                self.size += 1;
                return self;
            }
            let old_head = self.head.take();
            new_node.next = old_head;
            self.head = Some(Box::new(new_node));
            self.size += 1;
            return self;
        } else {
            let mut head = *self.head.unwrap();
            let mut new_head = Link::new();
            new_head.push(head.val);
            /* 每遍历一个节点，就将节点Copy到new_head中（需要实现Copy），update、remove、show的遍历运用相同逻辑 */
            for _i in 0..index - 1 {
                head = match head.next {
                    Some(x) => {
                        new_head.push(x.val);
                        *x
                    }
                    None => break,
                };
            }
            new_node.next = head.next;
            new_head.get_last().unwrap().next = Some(Box::new(new_node));
            new_head.size = self.size + 1;
            return new_head;
        }
    }

    fn remove(mut self, index: usize) -> Link<T> {
        if index > self.size {
            panic!("index: {} 已越界，链表元素长度为size: {}", index, self.size);
        }
        if index == 0 {
            let node = self.head.take();
            if node.is_some() {
                self.head = node.unwrap().next;
                self.size -= 1;
            }
            self
        } else {
            let mut head = *self.head.unwrap();
            let mut new_link = Link::new();
            /* 0..index是因为需要把欲删除之前的元素都存起来 */
            for _i in 0..index {
                new_link.push(head.val);
                head = *head.next.unwrap();
            }
            // 欲删除的元素 = 欲删除的元素.next;
            let head = head.next;
            /* 拿到最后一个节点，将next置为head，完成删除 */
            new_link.get_last().unwrap().next = head;
            new_link.size = self.size - 1;
            new_link
        }
    }

    fn update(mut self, index: usize, new_val: T) -> Link<T> {
        if index >= self.size {
            panic!("index: {} 已越界，链表元素长度为size: {}", index, self.size);
        }
        if index == 0 {
            let mut old_head = self.head.take();
            if old_head.is_none() {
                panic!("头节点不存在!");
            }
            old_head.as_mut().unwrap().val = new_val;
            self.head = old_head;
            return self;
        } else {
            let mut new_link = Link::new();
            let mut curr_node = *self.head.unwrap();
            for _i in 0..index {
                new_link.push(curr_node.val);
                curr_node = *curr_node.next.unwrap();
            }
            curr_node.val = new_val;
            new_link.get_last().unwrap().next = curr_node.to_option();
            new_link.size = self.size;
            return new_link;
        }
    }

    fn show(self) -> Link<T> {
        let mut new_link = Link::new();
        let mut curr_node = *self.head.unwrap();
        'target: for i in 0..self.size {
            new_link.push(curr_node.val);
            print!(" {} ", curr_node.val);
            if i != self.size - 1 {
                print!("->");
            }
            curr_node = {
                if curr_node.next.is_none() {
                    break 'target;
                } else {
                    *curr_node.next.unwrap()
                }
            };
        }
        println!();
        return new_link;
    }
}
