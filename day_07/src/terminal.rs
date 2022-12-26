enum Cmd {
    Ls(Ls),
    Cd(Cd),
}

// ls output
struct Ls(String);

struct Cd(String);
