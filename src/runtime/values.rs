use std::fmt::Debug;
use std::any::Any;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Null,
    Number,
}



pub trait RuntimeVal: Debug + Any {
    fn get_type(&self) -> ValueType;
    fn as_any(&self) -> &dyn Any; 
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

impl RuntimeVal for NumberVal {
    fn get_type(&self) -> ValueType {
        ValueType::Number
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl RuntimeVal for NullVal {
    fn get_type(&self) -> ValueType {
        ValueType::Null
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}



#[derive(Debug, Clone, Copy)]
pub struct NullVal;


