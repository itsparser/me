
// define a newtype we'll provide as context
// contexts are stored by their types, so it can be useful to create
// a new type to avoid confusion with other `WriteSignal<i32>`s we may have
// all types to be shared via context should implement `Clone`
#[derive(Clone, Debug, Default)]
pub struct ValueSetter {
    pub current_path: String
}


