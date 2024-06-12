pub type Callback<T> = fn(&T);

pub struct Property<T> {
    value: T,
    listeners: Vec<Callback<T>>,
}

impl<T> Property<T> {
    pub fn new(val: T) -> Self {
        Self {
            value: val,
            listeners: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, callback: Callback<T>) {
        self.listeners.push(callback);
    }

    pub fn set_value(&mut self, val: T) {
        self.value = val;
        self.listeners.iter().for_each(|listener| listener(&self.value));
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}