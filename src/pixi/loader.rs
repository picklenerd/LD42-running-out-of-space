pub struct Loader {
    paths: Vec<(String, String)>,
}

impl Loader {
    pub fn new() -> Self {
        Loader {
            paths: Vec::new(),
        }
    }

    pub fn add(&mut self, alias: &str, path: &str) -> &mut Self {
        js!{
            const alias = @{alias};
            const path = @{path};
            PIXI.loader.add(alias, path);
        };
        self.paths.push((String::from(alias), String::from(path)));
        self
    }

    pub fn load(&self, callback: impl Fn() + 'static) {
        js! {
            const cb = @{callback};
            const callback = () => {
                cb();
                cb.drop();
            };
            PIXI.loader.load(callback);
        };
    }
}