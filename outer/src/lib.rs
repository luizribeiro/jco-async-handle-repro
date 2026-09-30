mod bindings {
    wit_bindgen::generate!({ path: "../wit", world: "example:repro/outer" });
}

struct Component;

impl bindings::Guest for Component {
    async fn run() -> String {
        bindings::example::repro::api::run().await
    }
}

bindings::export!(Component with_types_in bindings);
