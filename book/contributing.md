# Contribute

There are some basic tips for someone who wants to contribute to evlis.

Firstly, welcome to post any questions about evlis via Github Issue, Email, etc. We will give you reaction as soon as possible. 

## Make A Change


1. Clone the repository on your local machine.
    
    When contributing to an open source project using GIT, you will have a copy or “clone” of the source code repository in your local development environment.Here, you can make your changes and commit them locally to create a revision history, allowing changes to be tracked and easily rolled back if required. 

    ```sh
    $ git clone https://github.com/elvisjs/elvis.git
    ```


2. Build the source 

    Evlis is implemented purely in Rust, So, please make sure you have installed [Rust](https://www.rust-lang.org/tools/install) before made contributions. There is not specific Rust version requirement, we recommend you to fetch the latest Rust version. 

    Once you have installed Rust, type the following in the terminal to start compiling:

    ```sh
    $ cd elvis
    $ cargo build
    ```

    The resulting binary can be found in `evlis/target/debug`.

## Raising A Pull Request

Once you feel satisfied with the changes, you can raise a pull request on Github.

Our maintainer will then approve the changes or request some changes before it get merged.

Happy contributions!