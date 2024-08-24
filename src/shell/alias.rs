use std::collections::HashMap;
use std::marker::PhantomData;

pub struct AliasManager<'a> {
    aliases: HashMap<String, String>,
    _marker: PhantomData<&'a ()>,
}

impl<'a> AliasManager<'a> {
    pub fn new() -> Self {
        AliasManager {
            aliases: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn add_alias(&mut self, name: &str, command: &str) {
        self.aliases.insert(name.to_string(), command.to_string());
    }

    pub fn remove_alias(&mut self, name: &str) {
        self.aliases.remove(name);
    }

    pub fn expand_alias(&'a self, command: &'a str) -> &'a str {
        if let Some(alias) = self.aliases.get(command) {
            alias
        } else {
            command
        }
    }
}
