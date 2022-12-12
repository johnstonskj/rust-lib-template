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
