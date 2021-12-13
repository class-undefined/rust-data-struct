use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
type Node<T> = Option<Rc<RefCell<LinkNode<T>>>>;
struct LinkNode<T> {
    val: T,
    next: Option<Rc<RefCell<LinkNode<T>>>>
}
// #[debug()]
impl <T: Display>LinkNode<T> {
    pub fn new(val: T) -> Self {
        Self {
            val, next: None
        }
    }

    pub fn from(val: T, next: Node<T>) -> Self {
        Self {val, next}
    }

    pub fn to_ref_cell(self) -> RefCell<LinkNode<T>> {
        RefCell::new(self)
    }

    pub fn to_rc(self) -> Rc<RefCell<LinkNode<T>>> {
        Rc::new(self.to_ref_cell())
    }

    pub fn to_option(self) -> Node<T> {
        Some(self.to_rc())
    }

    pub fn set_next(&mut self, next: Node<T>) -> () {
        if next.is_none() {
            self.next = None;
            return
        }
        let next_node = next.as_ref().unwrap();
        self.next = Some(Rc::clone(next_node));
    }
}

pub struct Link<T> {
    root: Node<T>,
    pub size: usize
}

impl <T: Display>Link<T> {
    pub fn new() -> Self {
        Self {root: None, size: 0}
    }

    /* 在指定索引插入元素 */
    pub fn insert(&mut self, begin: usize, val: T) -> () {
        if begin > self.size {
            panic!("插入的位置begin: {} 不合法", begin);
        }
        let mut node = LinkNode::new(val);
        /* 插入的是头节点 */
        if begin == 0 {
            /* 头节点为None，直接赋值 */
            if self.root.is_none() {
                self.root = node.to_option();
            }else {
                /* 不为None，则take */
                node.set_next(Some(self.root.take().unwrap().clone()));
                self.root = node.to_option();
            }
            self.size += 1;
            return;
        }
        /* 移动到插入元素的前一个位置，因为使用节点会被消耗，所以将用于遍历的节点进行一次克隆 */
        let mut curr = Rc::clone(self.root.as_ref().unwrap());
        for _i in 0..(begin - 1) {
            let _node = Rc::clone(&curr);
            curr = Rc::clone(&_node.as_ref().borrow_mut().next.as_ref().unwrap());
        }
        /* 获取欲插入元素的下一个节点 */
        let post = {
            let next_node = &curr.as_ref().borrow_mut().next;
            if next_node.is_none() {
                None
            } else {
                Some(Rc::clone(next_node.as_ref().unwrap()))
            }
        };
        node.set_next(post);
        curr.as_ref().borrow_mut().set_next(node.to_option());
        self.size += 1;
    }

    /* 删除指定索引元素 */
    pub fn remove(&mut self, index: usize) -> () {
        if index >= self.size {
            panic!("删除的位置index: {} 不合法", index);
        }
        if self.root.is_none() {
            panic!("容器内无元素删除!");
        }
        /* 处理当删除元素是头节点的情况 */
        if index == 0 {
            let node_option = self.root.take();
            let node = node_option.unwrap();
            let next_node_option = &node.as_ref().borrow().next;
            if next_node_option.is_none() {
                self.root = None;
                self.size -= 1;
                return ;
            }
            let next_node = next_node_option.as_ref().unwrap();
            self.root = Some(Rc::clone(next_node));
            self.size -= 1;
            return ;
            /* 处理删除元素非头节点的情况 */
        } else {
            /* 与insert的移动节点逻辑一致 */
            let mut curr_option = Rc::clone(self.root.as_ref().unwrap());
            for _i in 0..index - 1 {
                let next = Rc::clone(curr_option.borrow().next.as_ref().unwrap());
                curr_option = next;
            }
            let mut curr_node = curr_option.borrow_mut();
            /* 不能直接使用unwarp, 考虑欲删除节点的下一个节点是否为None */
            let next_node_option = {
                if curr_node.next.is_none() {
                    None
                } else {
                    let node = curr_node.next.as_ref().unwrap().borrow();
                    let next = node.next.as_ref().unwrap();
                    Some(Rc::clone(next))
                }
            };
            curr_node.next = next_node_option;
            self.size -= 1;
        }


    }

    /* 更新指定索引处的元素 */
    pub fn update(&mut self, index: usize, new_val: T) -> () {
        if index >= self.size {
            panic!("index: {} 已越界，链表元素长度为size: {}", index, self.size);
        }
        /* 与insert移动节点的逻辑一致 */
        let mut curr_node = Rc::clone(self.root.as_ref().unwrap());
        for _i in 0..index {
            let node = Rc::clone(curr_node.borrow().next.as_ref().unwrap());
            curr_node = node;
        }
        let mut need_update_node = curr_node.borrow_mut();
        need_update_node.val = new_val;
    }

    /* 输出元素 */
    pub fn show(&self) -> () {
        let mut curr = Rc::clone(self.root.as_ref().unwrap());
        for i in 0..self.size {
            /* 将当前的节点克隆一份，用于输出 */
            let rc_node = Rc::clone(&curr);
            let node = rc_node.as_ref().borrow();
            print!(" {} ", node.val);
            if i != self.size - 1 {
                print!("->");
            }
            if node.next.is_some() {
                curr = Rc::clone(node.next.as_ref().unwrap());
            }
        }
        println!();
    }
}