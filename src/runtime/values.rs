use std::fmt::Debug;
use std::any::Any;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Null,
    Number,
    Runtime
}



pub trait RuntimeValue: Debug + Any {
    fn get_type(&self) -> ValueType;
    fn get_value(&self) -> Box<dyn RuntimeValue>;
    fn as_any(&self) -> &dyn Any; 
}

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






