pub mod single_linked;


pub use crate::single_linked::box_no_ref::link::Link as no_ref_Link;
pub use crate::single_linked::box_ref::link::LinkList as ref_Link;
pub use crate::single_linked::rc_ref::link::Link as rc_ref_Link;

fn test1() {
    use single_linked::box_no_ref::link::LinkMethod;
    let mut link: no_ref_Link<i32> = no_ref_Link::new();
    link = link.insert(0, 1);
    link = link.insert(1, 2);
    link = link.insert(2, 3);
    /* 1 2 3 */
    link = link.remove(1);
    /* 1 3 */
    link = link.update(1, 10);
    /* 1 10 */
    link = link.show();
}

fn test2() {
    let mut link: ref_Link<i32> = ref_Link::new();
    link.insert(0, 10);
    /* 10 */

    link.insert(1, 11);
    /* 10 11 */

    link.insert(1, 100);
    /* 10 100 11 */

    link.insert(2, 12);
    /* 10 100 12 11 */

    link.update(3, 110);
    /* 10 100 12 110 */

    link.remove(1);
    /* 10 12 110*/

    link.show();
}

fn test3() {
    let mut link = rc_ref_Link::new();
    link.insert(0, 1);
    link.insert(1, 2);
    link.insert(2, 3);
    link.insert(3, 4);
    link.insert(4, 5);
    /* 1 2 3 4 5 */

    link.remove(1);
    /* 1 3 4 5 */

    link.update(2, 8);
    /* 1 3 8 5 */

    link.show();
}
fn main() {
    test1(); // no_ref_Link     Box无引用
    test2(); // ref_Link        Box + 引用
    test3(); // rc_ref_Link     Rc + RefCell
}
