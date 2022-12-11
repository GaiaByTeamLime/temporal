{ pkgs }:
with pkgs;

stdenv.mkDerivation {
    name = "temporal";
    buildInputs = [ 
        git 
        rust-bin.nightly."2022-12-10".default 
        rustfmt
        cargo-watch
        cargo 
        sqlx-cli
        openssl
        pkg-config
        jq
    ];
    shellHook = ''
        echo -e ""
        echo -e "    Welcome to the \x1b[33;1m⚙ rust\x1b[0m environment!"
        echo -e ""
        echo -e "    - Start a database with docker by running \x1b[1m./scripts/start_db.sh\x1b[0m"
        echo -e "    - To start the app run \x1b[1mcargo run\x1b[0m"
        echo -e "    - Use \x1b[1mcargo watch\x1b[0m to watch for changes"
        echo -e ""
    '';
}
