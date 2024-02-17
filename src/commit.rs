use git2::{Config, Repository, Signature};
use std::env::current_dir;

pub fn create_commit(message: &str) {
    let cwd = current_dir().expect("Could not get current working directory");

    let path: &std::path::Path = cwd.as_ref();
    let repo: Repository =
        Repository::open(path).expect("Could not open repository in current directory");

    let config = Config::open_default().expect("Unable to obtain git global config");
    let name = config
        .get_string("user.name")
        .expect("Unable to obtain user.name from git config");
    let email = config
        .get_string("user.email")
        .expect("Unable to obtain user.email from git config");
    let signature = Signature::now(&name, &email).expect("Could not create signature");

    let mut index = repo.index().expect("Unable to obtain index");
    let oid = index.write_tree().expect("Unable to obtain index tree");
    let tree = repo.find_tree(oid).expect("Could not find tree");
    let parent_commit = repo
        .head()
        .expect("Could not obtain HEAD")
        .resolve()
        .expect("Could not resolve HEAD")
        .peel_to_commit()
        .expect("Could not convert commit");

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )
    .expect("Could not create new commit");
}
