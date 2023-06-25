const INDENTATION: &str = "  ";

pub struct SerializationContext {
    pub indentation_level: u32,
}

impl SerializationContext {
    pub fn write_indentation(&self) -> String {
        let mut output = "".to_owned();
        for _ in 1..self.indentation_level {
            output.push_str(INDENTATION);
        }
        output
    }
}

// TODO: use a buffer and `write!(…)`?
pub trait Serialize {
    fn serialize_with_context(&self, context: &SerializationContext) -> String;
}
