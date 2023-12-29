use secrecy::SecretString;
pub trait Id {
    fn id(&self) -> u64;
    fn set_id(&mut self, id: u64);
}
#[macro_export]
macro_rules! impl_id {
    (for $($t:ty),+) => {
        $(impl Id for $t {
            fn id(&self) -> u64 {
                self.id
            }
            fn set_id(&mut self, id: u64) {
                self.id.zeroize();
                self.id = id;
            }
        })*
    }
}

pub trait Label {
    fn label(&self) -> &str;
    fn set_label(&mut self, label: String);
}
#[macro_export]
macro_rules! impl_label {
    (for $($t:ty),+) => {
        $(impl Label for $t {
            fn label(&self) -> &str {
                &self.label
            }
            fn set_label(&mut self, label: String) {
                self.label.zeroize();
                self.label = label;
            }
        })*
    }
}

pub trait Position {
    fn position(&self) -> u32;
    fn set_position(&mut self, position: u32);
}
#[macro_export]
macro_rules! impl_position {
    (for $($t:ty),+) => {
        $(impl Position for $t {
            fn position(&self) -> u32 {
                self.position
            }
            fn set_position(&mut self, position: u32) {
                self.position.zeroize();
                self.position = position;
            }
        })*
    }
}

pub trait Required {
    fn required(&self) -> bool;
    fn set_required(&mut self, required: bool);
}
#[macro_export]
macro_rules! impl_required{
    (for $($t:ty),+) => {
        $(impl Required for $t {
            fn required(&self) -> bool {
                self.required
            }
            fn set_required(&mut self, required: bool) {
                self.required.zeroize();
                self.required = required;
            }
        })*
    }
}

pub trait ToSecretString {
    fn to_secret_string(&self) -> SecretString;
}

#[macro_export]
macro_rules! impl_to_secret_string {
    (for $($t:ty),+) => {
        $(impl ToSecretString for $t {
            fn to_secret_string(&self) -> SecretString {
                SecretString::new(self.value.to_string())
            }
        })*
    }
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
