## `apt-listbugs-rs`

This project implements a subset of `apt-listbugs` (that is written in Ruby) specifically for Debian Unstable (Sid) in Rust. As a consequence it has 0 dependencies and runs slightly faster.

On the other side, it has 0 configuration but consists of less than 500 LOC (including tests).

### Internals

Once installed this package adds an apt hook in `/etc/apt/apt.conf.d/` directory that runs before installing any package(s), similar to `apt-listbugs`. This hook is the main binary of this package.

The hook is called with an `APT_HOOK_INFO_FD` enfironment variable set to a file descriptor of a file that contains apt metadata of installed packages.

The binary reads and parses this file and ends having a list of packages that are about to be installed. If you remove a package the hook also gets called, but then the list obviously is empty, and so the binary just exists with 0.

Once we have a list of packages to install we go to Debian bugtracker using SOAP API (there are no alternatives atm) and:

1. query the list of critical/serious/grave bugs for our list of packages using `get_bugs` SOAP method which returns a plain array of bug numbers.
2. query those bugs to get their description/severity using `list_bugs` SOAP method.

(For both of them fixtures are available in `/fixtures` directory in case you are intersted in the format of the API, `cargo test` makes sure we still handle them properly)

Then we print formatted bugs and ask for confirmation. If user says "yes, LGTM" we `exit(0)`, otherwise we `exit(1)`.

That's it.

## Releases

The package can be built manually using `cargo deb` (`cargo [b]install cargo-deb` is required), or check the latest release of this repo.

Be careful, by default `sudo apt install ./path/to/package.deb` doesn't install the APT hook (you should get warning about it), so feel free to place this file yourself in that case. If you install it from some kind of a PPA no additionals actions is required.
