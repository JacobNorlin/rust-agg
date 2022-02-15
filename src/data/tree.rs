use std::collections::HashMap;

use super::data_frame::{AnyValue, Field, Row};

//All references should live as long as the tree does I think ¯\_(ツ)_/¯
#[derive(Debug)]
pub struct Node<'a> {
    id: &'a AnyValue,
    rows: Vec<&'a Row>,
    children: Vec<Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn new(rows: Vec<&'a Row>, children: Vec<Node<'a>>, id: &'a AnyValue) -> Self {
        Node { rows, children, id }
    }

    pub fn _print(&self, d: usize) {
        let pre = "\t".repeat(d);
        println!("{}id:{:?}", pre, self.id);
        for row in &self.rows {
            println!("{}{:?}", pre, row);
        }

        for child in &self.children {
            child._print(d + 1);
        }
    }

    pub fn rows(&self) -> &Vec<&Row> {
        &self.rows
    }

    pub fn children(&self) -> &Vec<Node> {
        &self.children
    }
}

pub fn create_tree<'a>(row_pointers: Vec<&'a Row>, levels: &Vec<&'a Field>) -> Node<'a> {
    let root_children = create_tree_rec(&row_pointers, levels, 0);
    Node::new(row_pointers, root_children, &AnyValue::Null)
}

fn create_tree_rec<'a>(
    rows: &Vec<&'a Row>,
    levels: &Vec<&'a Field>,
    depth: usize,
) -> Vec<Node<'a>> {
    if depth == levels.len() {
        return vec![];
    }
    let level = levels.get(depth).unwrap();
    //Only iterate over the rows once instead of for each unique value...
    let rows_per_value = rows_with_values(rows, level);

    rows_per_value
        .into_iter()
        .map(|(val, rows)| {
            let children = create_tree_rec(&rows, levels, depth + 1);
            Node::new(rows.clone(), children, val)
        })
        .collect()
}

fn rows_with_values<'row>(
    rows: &Vec<&'row Row>,
    field: &Field,
) -> HashMap<&'row AnyValue, Vec<&'row Row>> {
    let mut map: HashMap<&AnyValue, Vec<&Row>> = HashMap::new();
    for row in rows {
        let row_val = field.read(row);
        if map.contains_key(row_val) {
            let row_vec = map.get_mut(row_val).unwrap();
            row_vec.push(row);
        } else {
            map.insert(row_val, vec![row]);
        }
    }
    map
}
