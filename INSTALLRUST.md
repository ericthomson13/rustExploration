# This is a quikstart guide on how to install Rust on a Mac

## If you are on a non-Mac/non-Unix machine see the official (docs)[https://forge.rust-lang.org/infra/other-installation-methods.html]

- On Unix based systems run the script below to install rustup:

```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```

- This will install rustup and cargo

### Rustup
Rustup is an installer and version management tool.  You can easily update rust by running:

```rustup update```

### Cargo
Cargo is a build and package manager.  There's a lot more to cargo but basic commands are:

- Build your project:
```cargo build```

- Run your project (this will also build then run):
```cargo run```

- Test your project:
```cargo test```

- Build project documentation:
```cargo doc```

- Publish your library to (crates.io)[https://crates.io/] which is where crates from ``` cargo ``` are pulled from:
```cargo publish```

- To explore more options and capabilities of cargo simply run:
```cargo``` or ```cargo help <command>```

If ```cargo``` commands don't seem to be working double check that you've installed cargo by running:
```cargo --version```

