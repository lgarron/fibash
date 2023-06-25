use crate::serialize::{SerializationContext, Serialize};

#[derive(Debug)]
pub struct Body {
    pub parts: Vec<BodyPart>,
}

impl Body {
    pub fn serialize(&self) -> String {
        self.serialize_with_context(&SerializationContext {
            indentation_level: 0,
        })
    }
}

impl Serialize for Body {
    fn serialize_with_context(&self, context: &SerializationContext) -> String {
        let mut output = "".to_string();
        for body_part in &self.parts {
            output.push_str(&body_part.serialize_with_context(context))
        }
        output
    }
}

#[derive(Debug)]
pub enum BodyPart {
    Newline(Newline),
    Command(Command),
}

impl Serialize for BodyPart {
    fn serialize_with_context(&self, context: &SerializationContext) -> String {
        match self {
            BodyPart::Newline(value) => value.serialize_with_context(context),
            BodyPart::Command(value) => value.serialize_with_context(context),
        }
    }
}

impl From<Newline> for BodyPart {
    fn from(value: Newline) -> Self {
        BodyPart::Newline(value)
    }
}

impl From<Command> for BodyPart {
    fn from(value: Command) -> Self {
        BodyPart::Command(value)
    }
}

#[derive(Debug)]
pub struct Newline {}

impl Serialize for Newline {
    fn serialize_with_context(&self, _context: &SerializationContext) -> String {
        "\n".to_owned()
    }
}

#[derive(Debug)]
pub struct Command {
    pub name: String, // TODO: dynamic name
    /** TODO:
     * - nested calculations that produce strings
     * - dynamic arguments
     */
    pub arguments: Vec<String>,
}

impl Serialize for Command {
    fn serialize_with_context(&self, context: &SerializationContext) -> String {
        let mut output = "(".to_owned();
        output.push_str(&context.write_indentation());
        output.push_str(&self.name);
        for argument in &self.arguments {
            output.push(' ');
            output.push_str(argument);
        }
        output.push(')');
        output
    }
}

// pub struct If {
//     pub then_clause: Body,
//     pub else_clause: Option<Body>,
// }
