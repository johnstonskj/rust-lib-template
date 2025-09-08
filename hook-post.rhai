file::rename("_github", ".github");

let crate_repo_url = variable::get("crate_repo_url");
let cliff_file = system::command("cat", ["cliff-in.toml"]);
cliff_file.replace("CRATE_REPO_URL", crate_repo_url);
file::write("cliff.toml", `${cliff_file}`);
file::delete("cliff-in.toml");
