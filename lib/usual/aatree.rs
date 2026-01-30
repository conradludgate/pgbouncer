
pub mod _uintptr_t_h {
    
    pub type uintptr_t = usize;
}

pub mod aatree_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AATree {
        pub root: *mut AANode,
        pub count: ::core::ffi::c_int,
        pub node_cmp: aatree_cmp_f,
        pub release_cb: aatree_walker_f,
    }
    
    pub type aatree_walker_f =
        Option<unsafe extern "C" fn(*mut AANode, *mut ::core::ffi::c_void) -> ()>;
    #[derive(Copy, Clone)]
    #[repr(C)]
    
    pub struct AANode {
        pub left: *mut AANode,
        pub right: *mut AANode,
        pub level: ::core::ffi::c_int,
    }
    
    pub type aatree_cmp_f =
        Option<unsafe extern "C" fn(uintptr_t, *mut AANode) -> ::core::ffi::c_int>;
    
    pub type AATreeWalkType = ::core::ffi::c_uint;
    
    pub const AA_WALK_POST_ORDER: AATreeWalkType = 2;
    
    pub const AA_WALK_PRE_ORDER: AATreeWalkType = 1;
    
    pub const AA_WALK_IN_ORDER: AATreeWalkType = 0;
    use super::_uintptr_t_h::uintptr_t;
}

pub mod _null_h {
    
    pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
    use super::_types_h::__DARWIN_NULL;
}

pub mod _types_h {
    
    pub const __DARWIN_NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null_mut::<::core::ffi::c_void>();
}
pub use self::_null_h::NULL;
pub use self::_types_h::__DARWIN_NULL;
pub use self::_uintptr_t_h::uintptr_t;
pub use self::aatree_h::{
    aatree_cmp_f, aatree_walker_f, AANode, AATree, AATreeWalkType, AA_WALK_IN_ORDER,
    AA_WALK_POST_ORDER, AA_WALK_PRE_ORDER,
};

pub type Tree = AATree;

pub type Node = AANode;

static mut _nil: AANode = unsafe {
    AANode {
        left: &raw const _nil as *mut AANode,
        right: &raw const _nil as *mut AANode,
        level: 0 as ::core::ffi::c_int,
    }
};
#[inline]

unsafe extern "C" fn skew(mut x: *mut Node) -> *mut Node {
    let mut y = (*x).left as *mut Node;
    if (*x).level == (*y).level && !std::ptr::eq(x, &raw const _nil) {
        (*x).left = (*y).right;
        (*y).right = x as *mut AANode;
        return y;
    }
    x
}
#[inline]

unsafe extern "C" fn split(mut x: *mut Node) -> *mut Node {
    let mut y = (*x).right as *mut Node;
    if (*x).level == (*(*y).right).level && !std::ptr::eq(x, &raw const _nil) {
        (*x).right = (*y).left;
        (*y).left = x as *mut AANode;
        (*y).level += 1;
        return y;
    }
    x
}

unsafe extern "C" fn rebalance_on_insert(mut current: *mut Node) -> *mut Node {
    split(skew(current))
}

unsafe extern "C" fn rebalance_on_remove(mut current: *mut Node) -> *mut Node {
    if (*(*current).left).level < (*current).level - 1 as ::core::ffi::c_int
        || (*(*current).right).level < (*current).level - 1 as ::core::ffi::c_int
    {
        (*current).level -= 1;
        if (*(*current).right).level > (*current).level {
            (*(*current).right).level = (*current).level;
        }
        current = skew(current);
        (*current).right = skew((*current).right as *mut Node) as *mut AANode;
        (*(*current).right).right = skew((*(*current).right).right as *mut Node) as *mut AANode;
        current = split(current);
        (*current).right = split((*current).right as *mut Node) as *mut AANode;
    }
    current
}

unsafe extern "C" fn insert_sub(
    mut tree: *mut Tree,
    mut current: *mut Node,
    mut value: uintptr_t,
    mut node: *mut Node,
) -> *mut Node {
    let mut cmp: ::core::ffi::c_int = 0;
    if std::ptr::eq(current, &raw const _nil) {
        (*node).right = &raw const _nil as *mut AANode;
        (*node).left = (*node).right;
        (*node).level = 1 as ::core::ffi::c_int;
        (*tree).count += 1;
        return node;
    }
    cmp = (*tree).node_cmp.expect("non-null function pointer")(value, current as *mut AANode);
    if cmp > 0 as ::core::ffi::c_int {
        (*current).right =
            insert_sub(tree, (*current).right as *mut Node, value, node) as *mut AANode;
    } else if cmp < 0 as ::core::ffi::c_int {
        (*current).left =
            insert_sub(tree, (*current).left as *mut Node, value, node) as *mut AANode;
    } else {
        return current;
    }
    rebalance_on_insert(current)
}
#[no_mangle]

pub unsafe extern "C" fn aatree_insert(
    mut tree: *mut Tree,
    mut value: uintptr_t,
    mut node: *mut Node,
) {
    (*tree).root = insert_sub(tree, (*tree).root as *mut Node, value, node) as *mut AANode;
}

unsafe extern "C" fn steal_leftmost(
    mut tree: *mut Tree,
    mut current: *mut Node,
    mut save_p: *mut *mut Node,
) -> *mut Node {
    if std::ptr::eq((*current).left, &raw const _nil) {
        *save_p = current;
        return (*current).right as *mut Node;
    }
    (*current).left = steal_leftmost(tree, (*current).left as *mut Node, save_p) as *mut AANode;
    rebalance_on_remove(current)
}

unsafe extern "C" fn drop_this_node(mut tree: *mut Tree, mut old: *mut Node) -> *mut Node {
    let mut new = &raw const _nil as *mut Node;
    if std::ptr::eq((*old).left, &raw const _nil) {
        new = (*old).right as *mut Node;
    } else if std::ptr::eq((*old).right, &raw const _nil) {
        new = (*old).left as *mut Node;
    } else {
        (*old).right = steal_leftmost(tree, (*old).right as *mut Node, &raw mut new) as *mut AANode;
        *new = *old;
    }
    if (*tree).release_cb.is_some() {
        (*tree).release_cb.expect("non-null function pointer")(
            old as *mut AANode,
            tree as *mut ::core::ffi::c_void,
        );
    }
    (*tree).count -= 1;
    new
}

unsafe extern "C" fn remove_sub(
    mut tree: *mut Tree,
    mut current: *mut Node,
    mut value: uintptr_t,
) -> *mut Node {
    let mut cmp: ::core::ffi::c_int = 0;
    if std::ptr::eq(current, &raw const _nil) {
        return current;
    }
    cmp = (*tree).node_cmp.expect("non-null function pointer")(value, current as *mut AANode);
    if cmp > 0 as ::core::ffi::c_int {
        (*current).right = remove_sub(tree, (*current).right as *mut Node, value) as *mut AANode;
    } else if cmp < 0 as ::core::ffi::c_int {
        (*current).left = remove_sub(tree, (*current).left as *mut Node, value) as *mut AANode;
    } else {
        current = drop_this_node(tree, current);
    }
    rebalance_on_remove(current)
}
#[no_mangle]

pub unsafe extern "C" fn aatree_remove(mut tree: *mut Tree, mut value: uintptr_t) {
    (*tree).root = remove_sub(tree, (*tree).root as *mut Node, value) as *mut AANode;
}

unsafe extern "C" fn walk_sub(
    mut current: *mut Node,
    mut wtype: AATreeWalkType,
    mut walker: aatree_walker_f,
    mut arg: *mut ::core::ffi::c_void,
) {
    if std::ptr::eq(current, &raw const _nil) {
        return;
    }
    match wtype as ::core::ffi::c_uint {
        0 => {
            walk_sub((*current).left as *mut Node, wtype, walker, arg);
            walker.expect("non-null function pointer")(current as *mut AANode, arg);
            walk_sub((*current).right as *mut Node, wtype, walker, arg);
        }
        2 => {
            walk_sub((*current).left as *mut Node, wtype, walker, arg);
            walk_sub((*current).right as *mut Node, wtype, walker, arg);
            walker.expect("non-null function pointer")(current as *mut AANode, arg);
        }
        1 => {
            walker.expect("non-null function pointer")(current as *mut AANode, arg);
            walk_sub((*current).left as *mut Node, wtype, walker, arg);
            walk_sub((*current).right as *mut Node, wtype, walker, arg);
        }
        _ => {}
    };
}
#[no_mangle]

pub unsafe extern "C" fn aatree_walk(
    mut tree: *mut Tree,
    mut wtype: AATreeWalkType,
    mut walker: aatree_walker_f,
    mut arg: *mut ::core::ffi::c_void,
) {
    walk_sub((*tree).root as *mut Node, wtype, walker, arg);
}
#[no_mangle]

pub unsafe extern "C" fn aatree_destroy(mut tree: *mut Tree) {
    walk_sub(
        (*tree).root as *mut Node,
        AA_WALK_POST_ORDER,
        (*tree).release_cb,
        tree as *mut ::core::ffi::c_void,
    );
    (*tree).root = &raw const _nil as *mut AANode;
    (*tree).count = 0 as ::core::ffi::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn aatree_init(
    mut tree: *mut Tree,
    mut cmpfn: aatree_cmp_f,
    mut release_cb: aatree_walker_f,
) {
    (*tree).root = &raw const _nil as *mut AANode;
    (*tree).count = 0 as ::core::ffi::c_int;
    (*tree).node_cmp = cmpfn;
    (*tree).release_cb = release_cb;
}
#[no_mangle]

pub unsafe extern "C" fn aatree_search(mut tree: *mut Tree, mut value: uintptr_t) -> *mut AANode {
    let mut current = (*tree).root as *mut Node;
    while !std::ptr::eq(current, &raw const _nil) {
        let mut cmp =
            (*tree).node_cmp.expect("non-null function pointer")(value, current as *mut AANode);
        if cmp > 0 as ::core::ffi::c_int {
            current = (*current).right as *mut Node;
        } else if cmp < 0 as ::core::ffi::c_int {
            current = (*current).left as *mut Node;
        } else {
            return current as *mut AANode;
        }
    }
    ::core::ptr::null_mut::<AANode>()
}
