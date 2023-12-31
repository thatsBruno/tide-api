use std::collections::HashMap;
use std::sync::Arc;
use async_std::sync::RwLock;

#[derive(serde::Deserialize, serde::Serialize)]
struct Wizard {
    name: String,
    level: u8,
}

struct Repository {
    wizards: HashMap<String, Wizard>,
}

impl Repository {
    fn new() -> Self { Self { wizards: HashMap::new() } }
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
// async fn read_all(_req: Request<()>) -> tide::Result<tide::Body> {
//   let wizards = vec![
//       Wizard {
//           name: "Gandalf".to_string(),
//           level: 100,
//       },
//       Wizard {
//           name: "Merlin".to_string(),
//           level: 65,
//       },
//   ];
//
//   Ok(tide::Body::from_json(&wizards)?)
//}

// async fn create(mut req: Request<()>) -> tide::Result<String> {
//    let wizard: Wizard = req.body_json().await?;
//    Ok(format!("{} is level {}!", wizard.name, wizard.level))
// }


#[tokio::main]
async fn main() -> tide::Result<()> {
    femme::start();
    let state = Arc::new(RwLock::new(Repository::new()));
    let mut app = tide::with_state(state);

    app.with( tide::log::LogMiddleware::new());

    // post json { name: foo, level: bar }
    // return wizards in json format
    app.at("/wizards").post(create);
    app.at("/wizards").get(get);

    // returns file
    // app.at("/file").serve_file("hello.txt").unwrap();

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
