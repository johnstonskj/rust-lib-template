let year = system::date().year;
variable::set("license-year", `${year}`);

let user_name = system::command("id", ["-un"]);
let gh_user_name = variable::prompt("Github user name", `${user_name}`);
variable::set("gh-user-name", gh_user_name);

variable::set(
    "gh-repo-name",
    variable::get("gh-repo-prefix") +
    variable::get("project-name")
);

variable::set(
    "gh-repo",
    variable::get("gh-user-name") +
    "/" +
    variable::get("gh-repo-name")
);

variable::set(
    "gh-repo-url",
    "https://github.com/" +
    variable::get("gh-repo")
);

let crate_name = variable::get("crate_name");
variable::set("crate_url", `https//crates.io/crates/${crate_name}`);
variable::set("crate_cov_url", `https://codecov.io/gh/${crate_name}`);
variable::set("crate_doc_url", `https://docs.rs/${crate_name}`);
variable::set("crate_repo_url", variable::get("gh-repo-url"));

file::rename("_github", ".github");