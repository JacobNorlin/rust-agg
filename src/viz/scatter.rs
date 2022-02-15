use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{console, CanvasRenderingContext2d};

use crate::{
    create_data_frame,
    data::{
        aggregation::{Aggregate, SumAggregate},
        data_frame::{DataFrame, Field},
        tree::{create_tree, Node},
    },
};

#[derive(Debug)]
pub struct ScatterLayout {
    items: Vec<ScatterItem>,
}

impl ScatterLayout {
    pub fn new(root: &Node, x_agg: &Aggregate, y_agg: &Aggregate) -> Self {
        let mut stack = vec![root];
        let mut scatter_items: Vec<ScatterItem> = vec![];

        let (min_x, max_x) = get_min_and_max(root, x_agg);
        let (min_y, max_y) = get_min_and_max(root, y_agg);

        println!("{}", max_y);

        while stack.len() > 0 {
            let curr = stack.pop().unwrap();

            if curr.children().len() > 0 {
                for child in curr.children() {
                    stack.push(child);
                }
                continue;
            }

            let x_val = x_agg.get_value(curr);
            let y_val = y_agg.get_value(curr);

            let x_norm = (x_val - min_x) / (max_x - min_x);
            let y_norm = (y_val - min_y) / (max_y - min_y);

            let scatter_item = ScatterItem::new(x_norm, y_norm, 1f32);
            scatter_items.push(scatter_item);
        }

        ScatterLayout {
            items: scatter_items,
        }
    }
}

fn get_min_and_max(node: &Node, agg: &Aggregate) -> (f32, f32) {
    let mut stack = vec![node];

    let mut min: f32 = f32::MAX;
    let mut max = f32::MIN;
    while stack.len() > 0 {
        let curr = stack.pop().unwrap();

        if curr.children().len() > 0 {
            for child in curr.children() {
                stack.push(child);
            }
            continue;
        }

        let val = agg.get_value(curr);
        min = f32::min(min, val);
        max = f32::max(max, val);
    }

    return (min, max);
}

#[derive(Debug)]
struct ScatterItem {
    x: f32,
    y: f32,
    size: f32,
}

impl ScatterItem {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        ScatterItem { x, y, size }
    }
}

#[wasm_bindgen]
pub struct ScatterWebGlRenderer {
    frame: DataFrame,
}

#[wasm_bindgen]
impl ScatterWebGlRenderer {
    pub fn new(raw_data: String, first_row_header: bool) -> Self {
        let frame = create_data_frame(raw_data, first_row_header);

        ScatterWebGlRenderer { frame }
    }

    pub fn render(
        &self,
        canvas_id: String,
        canvas_width: u32,
        canvas_height: u32,
        level_col_names_str: String,
        x_col_name: String,
        y_col_name: String,
    ) {
        let schema = &self.frame.schema();

        let level_col_names: Vec<String> = level_col_names_str
            .split(',')
            .map(|v| v.to_string())
            .collect();

        let levels: Vec<&Field> = level_col_names
            .iter()
            .map(|name| match schema.get_field_by_name(name) {
                Some(field) => field,
                None => panic!("Unknown field"),
            })
            .collect();

        let x_field = schema.get_field_by_name(&x_col_name).unwrap();
        let y_field = schema.get_field_by_name(&y_col_name).unwrap();

        let x_agg = Aggregate::Sum(SumAggregate::new(x_field));
        let y_agg = Aggregate::Sum(SumAggregate::new(y_field));

        console::time_with_label("tree");
        let tree = create_tree(self.frame.row_pointers(), &levels);
        console::time_end_with_label("tree");

        let layout = ScatterLayout::new(&tree, &x_agg, &y_agg);

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(&canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_width(canvas_width.into());
        canvas.set_height(canvas_height.into());

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();
        for item in &layout.items {
            let actual_x: f64 = (item.x * (canvas_width as f32)).into();
            let actual_y: f64 = (item.y * (canvas_height as f32)).into();

            context.rect(actual_x, actual_y, 4.0, 4.0);
        }

        context.close_path();

        context.fill();
    }
}
