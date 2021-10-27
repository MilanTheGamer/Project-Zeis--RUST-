use std::collections::HashMap;

pub struct QueryString<'buf> {
    data:HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key:&str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(string:&'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in string.split('&'){
            let mut key = sub_str;
            let mut value = "";

            if let Some(index) = sub_str.find('=') {
                key = &sub_str[..index];
                value = &sub_str[index+1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_value) => {
                        *existing = Value::Multiple(vec![prev_value,value]);
                    },
                    Value::Multiple(vec) => vec.push(value)
                })
                .or_insert(Value::Single(value));
        }

        QueryString {data}
    }
}