type TreeNode<K, V> = Option<Box<Node<K, V>>>;
#[derive(Debug)]
pub struct Node<K, V: std::fmt::Display> {
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    key: K,
    value: V,
}
