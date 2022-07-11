type NodeId = u32;
type ValueType = f32;

// tentative functional traits
trait Functional {
    fn id(&self) -> NodeId;
    fn out_val(&self) -> ValueType;
    fn in_vals(&self) -> &Vec<ValueType>;
    fn in_ids(&self) -> &Vec<NodeId>;
}