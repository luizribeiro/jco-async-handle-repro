use bindings::example::repro::host::{make, root};

mod bindings {
    wit_bindgen::generate!({ path: "../wit", world: "example:repro/inner" });
}

struct Component;

impl bindings::exports::example::repro::api::Guest for Component {
    async fn run() -> String {
        let a = root();
        let b = make("b".into()).await;
        format!("{} {}", a.name(), b.name())
    }
}

bindings::export!(Component with_types_in bindings);
