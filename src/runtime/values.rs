use std::fmt::Debug;
use std::any::Any;
use std::rc::Rc;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Null,
    Number,
    Runtime,
    Boolean,
}
pub trait RuntimeValue: Debug + Any {
    fn get_type(&self) -> ValueType;
    fn get_value(&self) -> Box<dyn RuntimeValue>;
    fn as_any(&self) -> &dyn Any; 
}

// region:    --- ValueConstructors



pub fn makeruntime () -> Rc<dyn RuntimeValue> {
    Rc::new(RuntimeVal::new())
}
pub fn makenumber(value: f64) -> Rc<dyn RuntimeValue> {
    Rc::new(NumberVal::new(value))
}
pub fn makebool(value: Option<bool>) -> Rc<dyn RuntimeValue> {
    let default_value = value.unwrap_or(true);
    Rc::new(BoolVal::new(default_value))
}


pub fn makenull() -> Rc<dyn RuntimeValue> {
    Rc::new(NullVal)
}

// endregion: --- ValueConstructors









// region:    --- RuntimeVal

#[derive(Debug, Clone, Copy)]
pub struct RuntimeVal{
    
}

impl RuntimeVal{

    pub fn new() -> Self {
        RuntimeVal{}
    }

    pub fn value(&self) -> RuntimeVal {
        RuntimeVal{}
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl RuntimeValue for RuntimeVal {
    fn get_type(&self) -> ValueType {
        ValueType::Runtime
    }

    fn get_value(&self) -> Box<dyn RuntimeValue> {
        Box::new(RuntimeVal::new())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// endregion: --- RuntimeVal

// region:    --- NumberVal
#[derive(Debug, Clone, Copy)]
pub struct NumberVal {
    value: f64,
}

impl NumberVal {
    pub fn new(value: f64) -> Self {
        NumberVal { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl RuntimeValue for NumberVal {
    fn get_type(&self) -> ValueType {
        ValueType::Number
    }

    fn get_value(&self) -> Box<dyn RuntimeValue> {
        Box::new(NumberVal::new(self.value))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// endregion: --- NumberVal

// region:    --- BoolVal
#[derive(Debug, Clone, Copy)]
pub struct BoolVal {
    value: bool,
}

impl BoolVal {
    pub fn new(value: bool) -> Self {
        BoolVal { value }
    }

    pub fn value(&self) -> bool {
        self.value
    }

    fn as_any(&self) -> &dyn Any { self }
}

impl RuntimeValue for BoolVal {
    fn get_type(&self) -> ValueType {
        ValueType::Boolean
    }

    fn get_value(&self) -> Box<dyn RuntimeValue> {
        Box::new(BoolVal::new(self.value))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// endregion: --- BoolVal

// region:    --- NullVal
#[derive(Debug, Clone, Copy)]
pub struct NullVal;

impl RuntimeValue for NullVal {
    fn get_type(&self) -> ValueType {
        ValueType::Null
    }

    fn get_value(&self) -> Box<dyn RuntimeValue> {
        Box::new(NullVal)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
// endregion: --- NullVal





