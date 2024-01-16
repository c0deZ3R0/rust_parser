// region:    --- Modules

pub type Result<T> = core::result::Result<T, Error>;

pub type Error = Box<dyn std::error::Error>; // For early dev. Later we will use our own error type.

// endregion: --- Modules

pub mod consts {
    pub const DEFAULT_SYSTEM_MOCK: &str = r#"    
    
        10 + 100 * ( 20 - 30 ) / 40

    "#;




}