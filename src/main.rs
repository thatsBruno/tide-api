use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(serde::Deserialize, serde::Serialize)]
struct Wizard {
    name: String,
    level: u8,
}

struct Repository {
    wizards: HashMap<String, Wizard>,
}

impl Repository {
    fn new() -> Self {
        Self {
            wizards: HashMap::new(),
        }
    }
}

type State = Arc<RwLock<Repository>>;

async fn create(mut req: tide::Request<State>) -> tide::Result {
    let wizard: Wizard = req.body_json().await?;
    let state = req.state();
    let mut repo = state.write().await;

    repo.wizards.insert(wizard.name.clone(), wizard);

    Ok(tide::Response::new(200))
}

async fn get(req: tide::Request<State>) -> tide::Result {
    let state = req.state();
    let repo = &state.read().await;

    let resp = tide::Response::builder(200)
        .body(tide::Body::from_json(&repo.wizards)?)
        .build();

    Ok(resp)
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();
    let state = Arc::new(RwLock::new(Repository::new()));
    let mut app = tide::with_state(state);

    app.at("/wizards").post(create);
    app.at("/wizards").get(get);

    // returns file
    // app.at("/file").serve_file("hello.txt").unwrap();

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
