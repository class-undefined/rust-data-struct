use std::fmt::{Debug, Display};

pub type Node<T> = Box<LinkNode<T>>;

pub struct LinkNode<T> {
    pub val: T,
    pub next: Option<Node<T>>,
}
impl<T: Debug + Display> LinkNode<T> {
    pub fn new(val: T) -> Self {
        LinkNode { val, next: None }
    }

    pub fn from(val: T, next: Option<Node<T>>) -> Self {
        LinkNode { val, next }
    }
}

pub struct LinkList<T: Display> {
    head: Option<Box<LinkNode<T>>>,
    len: u32,
}
impl<T: Display + Debug> LinkList<T> {
    pub fn new() -> Self {
        LinkList { head: None, len: 0 }
    }

    /* 向后追加 */
    pub fn push_back(&mut self, e: T) -> () {
        let node = LinkNode::new(e);
        if self.head.is_none() {
            self.head = Some(Box::new(node));
            self.len += 1;
            return;
        }
        let mut cursor = self.head.as_mut().unwrap();
        /* 如果next不为none */
        while cursor.next.is_some() {
            let node_next = cursor.next.as_mut().unwrap();
            cursor = node_next;
        }
        cursor.next = Some(Box::new(node));
        self.len += 1;
    }

    /* 弹出尾元素 */
    pub fn pop_back(&mut self) -> Option<Node<T>> {
        match self.head.as_mut() {
            None => None,
            Some(mut curr) => {
                while curr.next.is_some() && curr.next.as_ref().unwrap().next.is_some() {
                    curr = curr.next.as_mut().unwrap();
                }
                self.len -= 1;
                match curr.next {
                    Some(_) => Some(curr.next.take().unwrap()),
                    None => Some(self.head.take().unwrap()),
                }
            }
        }
    }

    /* 删除指定索引元素 */
    pub fn remove(&mut self, index: u32) -> bool {
        if index >= self.size() {
            return false;
        }
        match self.head.as_mut() {
            None => false,
            Some(mut curr) => {
                /* get pre_node */
                for _i in 0..index - 1 {
                    curr = curr.next.as_mut().unwrap();
                }
                match curr.next.take() {
                    None => false,
                    Some(node) => {
                        curr.next = node.next;
                        self.len -= 1;
                        true
                    }
                }
            }
        }
    }

    /* 在指定索引插入元素 */
    pub fn insert(&mut self, index: u32, val: T) -> bool {
        if index > self.size() {
            return false;
        }
        /* 如果index == 0，直接take */
        if index == 0 {
            let node = self.head.take();
            self.head = Some(Box::new(LinkNode::new(val)));
            self.head.as_mut().unwrap().next = node;
            self.len += 1;
            return true;
        }
        match self.head.as_mut() {
            None => {
                self.head = Some(Box::new(LinkNode::new(val)));
                self.len += 1;
                return true;
            }
            Some(mut curr) => {
                /* 获取需要插入元素位置的前一个位置元素 */
                for _i in 0..index - 1 {
                    curr = curr.next.as_mut().unwrap();
                }
                // 拿到后缀节点
                let post = curr.next.take();
                curr.next = Some(Box::new(LinkNode::new(val)));
                // curr.next.as_mut(): 获取curr.next的修改权 unwrap: 获取Option中的非None数据
                curr.next.as_mut().unwrap().next = post;
                self.len += 1;
                return true;
            }
        }
    }

    /* 更新指定索引处的元素 */
    pub fn update(&mut self, index: u32, new_val: T) -> () {
        if index >= self.len {
            panic!("index: {} 已越界，链表元素长度为size: {}", index, self.len);
        }
        let mut curr_node = &mut self.head;
        for _i in 0..index {
            let next = &mut curr_node.as_mut().unwrap().next;
            curr_node = next;
        }
        let mut node = curr_node.as_mut().unwrap();
        node.val = new_val;
    }

    /* 通过向量追加元素 */
    pub fn push_from_vec(&mut self, vec: Vec<T>) -> () {
        for e in vec {
            self.push_back(e);
        }
    }

    /* 输出元素 */
    pub fn show(&mut self) -> () {
        let mut cursor = self.head.as_mut().unwrap();
        for _i in 0..self.len {
            print!(" {} ", cursor.val);
            if cursor.next.is_some() {
                print!("->");
                cursor = cursor.next.as_mut().unwrap();
            }
        }
        println!();
    }

    /* 获取Link元素长度 */
    pub fn size(&mut self) -> u32 {
        self.len
    }

    /* Link是否为空 */
    pub fn empty(&self) -> bool {
        self.head.is_none()
    }
}
