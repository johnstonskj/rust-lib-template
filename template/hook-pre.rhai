if ! variable::is_set("license-year") {
    let year = system::date().year;
    variable::set("license-year", `${year}`);
}

if ! variable::is_set("gh-user-name") {
    let user_name = system::command("id", ["-un"]);
    let gh_user_name = variable::prompt("Github user name", `${user_name}`);
    variable::set("gh-user-name", gh_user_name);
}

if ! variable::is_set("gh-repo-name") {
    variable::set(
        "gh-repo-name",
        variable::get("gh-repo-prefix") +
        variable::get("project-name")
    );
}

if ! variable::is_set("gh-repo") {
    variable::set(
        "gh-repo",
        variable::get("gh-user-name") +
        "/" +
        variable::get("gh-repo-name")
    );
}

if ! variable::is_set("gh-repo-url") {
    variable::set(
        "gh-repo-url",
        "https://github.com/" +
        variable::get("gh-repo")
    );
}

let crate_name = variable::get("crate_name");
let github_repo = variable::get("gh-repo");

if ! variable::is_set("crate_url") {
    variable::set("crate_url", `https://crates.io/crates/${crate_name}`);
}
if ! variable::is_set("crate_cov_url") {
    variable::set("crate_cov_url", `https://app.codecov.io/github/${github_repo}`);
}
if ! variable::is_set("crate_doc_url") {
    variable::set("crate_doc_url", `https://docs.rs/${crate_name}`);
}
if ! variable::is_set("crate_repo_url") {
    variable::set("crate_repo_url", variable::get("gh-repo-url"));
}
