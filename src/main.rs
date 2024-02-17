mod commit;
mod read_message;

fn main() {
    let commit_message = read_message::read_message().expect("mira macho");
    commit::create_commit(&commit_message);
}
