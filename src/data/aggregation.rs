use super::{
    data_frame::{AnyValue, Field},
    tree::Node,
};

pub enum Aggregate<'a> {
    Sum(SumAggregate<'a>),
}

impl Aggregate<'_> {
    pub fn get_value(&self, node: &Node) -> f32 {
        use Aggregate::*;
        match self {
            Sum(sum_agg) => sum_agg.aggregate(node),
        }
    }
}

pub struct SumAggregate<'a> {
    field: &'a Field,
}

impl<'a> SumAggregate<'a> {
    pub fn new(field: &'a Field) -> Self {
        SumAggregate { field }
    }
    pub fn aggregate(&self, root: &Node) -> f32 {
        let mut sum = 0f32;
        sum = root.rows().iter().fold(sum, |acc, row| {
            let val = &self.field.read(row);
            match val {
                AnyValue::Number(v) => acc + v,
                _ => acc,
            }
        });
        sum
    }
}
