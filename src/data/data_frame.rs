use std::{
    cmp::Ordering,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    panic,
};

pub struct DataFrame {
    schema: Schema,
    rows: Vec<Row>,
}

impl DataFrame {
    pub fn new(data: &Vec<Vec<&str>>, first_row_header: bool) -> Self {
        let schema = Schema::from(data);
        let rows = data
            .iter()
            .cloned()
            .skip(if first_row_header { 1 } else { 0 })
            .map(|row| {
                return Row::from(row);
            })
            .collect();
        DataFrame {
            schema: schema,
            rows: rows,
        }
    }

    pub fn schema(&self) -> &Schema {
        &self.schema
    }

    pub fn row_pointers(&self) -> Vec<&Row> {
        self.rows.iter().map(|row| row).collect()
    }
}

#[derive(Debug)]
pub struct Row {
    values: Vec<AnyValue>,
}

impl Row {
    pub fn new(values: Vec<AnyValue>) -> Self {
        Row { values: values }
    }

    pub fn value<'s>(&self, i: usize) -> &AnyValue {
        match self.values.get(i) {
            Some(v) => v,
            None => panic!("Index out of bounds"),
        }
    }
}

impl From<Vec<&str>> for Row {
    fn from(r: Vec<&str>) -> Self {
        let values = r
            .iter()
            .map(|v| {
                return AnyValue::from(*v);
            })
            .collect();
        Row::new(values)
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(").unwrap();
        for value in &self.values {
            write!(f, "{},", value).unwrap();
        }
        write!(f, ")")
    }
}

#[derive(Debug)]
pub struct Schema {
    fields: Vec<Field>,
}

impl Schema {
    pub fn new(fields: Vec<Field>) -> Self {
        Schema { fields: fields }
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub fn get_field(&self, i: usize) -> Option<&Field> {
        self.fields.get(i)
    }

    pub fn get_field_by_name(&self, name: &String) -> Option<&Field> {
        self.fields.iter().find(|field| field.name() == name)
    }
}

impl From<&Vec<Vec<&str>>> for Schema {
    fn from(v: &Vec<Vec<&str>>) -> Self {
        let header = v.get(0).unwrap();

        //just assume the data isn't garbage
        let first_row = v.get(1).unwrap();
        let fields = first_row
            .iter()
            .enumerate()
            .map(|(col_num, val)| {
                let field_name = header.get(col_num).unwrap();

                //From impl magic
                Field::new(field_name, DataType::from(*val), col_num)
            })
            .collect();

        Schema::new(fields)
    }
}

impl Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(").unwrap();
        for field in self.fields() {
            write!(f, "{},", field.name()).unwrap();
        }
        write!(f, ")\n")
    }
}

#[derive(Debug)]
pub struct Field {
    name: String,
    data_type: DataType,
    index: usize,
}

impl Field {
    pub fn new(name: &str, data_type: DataType, index: usize) -> Self {
        Field {
            name: name.to_string(),
            data_type: data_type,
            index: index,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn data_type(&self) -> &DataType {
        &self.data_type
    }

    pub fn read<'s>(&self, row: &'s Row) -> &'s AnyValue {
        row.value(self.index)
    }
}

#[derive(Debug)]
pub enum AnyValue {
    Number(f32),
    Utf8(String),
    Null,
}

impl Eq for AnyValue {}

impl PartialEq for AnyValue {
    fn eq(&self, other: &Self) -> bool {
        use AnyValue::*;
        match (self, other) {
            (Number(l), Number(r)) => l == r,
            (Utf8(l), Utf8(r)) => l == r,
            _ => false,
        }
    }
}

impl Ord for AnyValue {
    fn cmp(&self, other: &Self) -> Ordering {
        use AnyValue::*;
        match (self, other) {
            (Number(l), Number(r)) => l.partial_cmp(r).unwrap_or(Ordering::Equal), //yolo
            (Utf8(l), Utf8(r)) => l.cmp(r),
            _ => Ordering::Equal,
        }
    }
}

impl PartialOrd for AnyValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use AnyValue::*;
        match (self, other) {
            (Number(l), Number(r)) => l.partial_cmp(r),
            (Utf8(l), Utf8(r)) => Some(l.cmp(r)),
            _ => None,
        }
    }
}

impl From<&str> for AnyValue {
    fn from(a: &str) -> Self {
        let as_string = String::from(a);
        let as_num = as_string.parse::<f32>();

        match as_num {
            Ok(num) => AnyValue::Number(num),
            Err(_e) => AnyValue::Utf8(as_string),
        }
    }
}

impl Hash for AnyValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use AnyValue::*;
        match &self {
            Utf8(v) => state.write(v.as_bytes()),
            Number(_v) => panic!("Can't hash floats"),
            Null => state.write_u64(u64::MAX / 2 + 135123),
        }
    }
}

impl Display for AnyValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use AnyValue::*;
        match &self {
            Utf8(v) => write!(f, "{}", v),
            Number(v) => write!(f, "{}", v),
            Null => write!(f, "null"),
        }
    }
}

#[derive(Debug)]
pub enum DataType {
    Number,
    Utf8,
    Null,
}

impl From<&str> for DataType {
    fn from(a: &str) -> Self {
        let v = String::from(a);
        let as_num = v.parse::<f32>();

        match as_num {
            Ok(_) => DataType::Number,
            Err(_) => DataType::Utf8,
        }
    }
}
