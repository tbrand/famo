#[derive(Debug, Fail)]
pub enum FamoError {
    #[fail(display = "{}", description)]
    MissedOption { description: String },
}
