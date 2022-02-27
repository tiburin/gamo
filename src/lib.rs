pub mod app;
use app::sync::{self, Connector};
use app::Sphere;
use std::{fs, path::Path};

pub fn run(sphere: &Sphere) -> Connector {
    let connector = Connector::new().setup(&sphere);
    sync::inner(&connector, sphere);
    connector
}

pub fn write_release(connector: &Connector, sphere: &Sphere) {
    let mut report: Vec<String> = Vec::new();
    sync::Release::write(&connector, &sphere, &mut report);
    sync::Release::write_report(report);
}

pub fn write_build(sphere: &Sphere, keys: bool) {
    let root = Path::new("build");

    if !root.is_dir() {
        fs::create_dir(root).unwrap();
    };

    for (tipo, list) in &sphere.vocabulary.data {
        let contents = list
            .iter()
            .enumerate()
            .fold(String::new(), |mut acc, (rank, mas)| {
                let content = match keys {
                    false => format!("{}\n", mas.word),
                    true => format!("{},{},s\n", rank + 1, mas.word),
                };

                acc.push_str(&content);
                acc
            });

        let path = root.join(format!("{}.on", tipo));

        if contents.len() > 0 {
            fs::write(path, contents).unwrap();
        }
    }
}

pub fn start() -> Sphere {
    Sphere::new().setup().mas()
}
