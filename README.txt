An educational attempt to implement malloc.

* https://www.cs.cmu.edu/~bryant/pubdir/sigcse18.pdf
* https://www.gnu.org/software/libc/manual/html_node/Replacing-malloc.html

==========
LD_PRELOAD
==========

So I do some disgusting things with gdb and LD_PRELOAD.

* Get parent pid of calling `cargo` command from build.rs using nix::getppid
* Get parent of `cargo` from `/proc/$ppid/status`, a.k.a. pppid (parent-parent-pid)
* Attach to that pid with gdb and set LD_PRELOAD to the appropriate environment .so file, so that the konfiscator implementation of malloc supersedes the stdlib one

This way, build.rs modifies LD_PRELOAD in the calling shell. It's only tested on Linux.

You can toggle this by setting `MODIFY_ENV_VARS`:

    sevagh:konfiscator $ cargo clean && cargo build >& /dev/null
    sevagh:konfiscator $ echo $LD_PRELOAD
    sevagh:konfiscator $ cargo clean && MODIFY_ENV_VARS=yes cargo build >& /dev/null
    sevagh:konfiscator $ echo $LD_PRELOAD
    /home/sevagh/repos/konfiscator/target/debug/libkonfiscator.so
