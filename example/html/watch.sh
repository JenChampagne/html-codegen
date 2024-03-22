echo 'Beginning cargo watch over websocket port 3232.'
echo 'Note that this script is meant to be run from ./html'
echo
echo 'Run the following to see compilation: websocat ws://127.0.0.1:3232/'
echo

# Delete previous iterations of html crate.
rm ../dist/libexample_html-d*.so

# Breakdown of steps in the following command:
# 1. `unbuffer` is used to make the output of cargo watch immediately pipe
#    into websocat so that rebuilds can be be served to websocket listeners.
#
#    Those build messages can be used by listeners to detect a code change.
#
#    If you don't need auto-reloading, you could remove `unbuffer` and the
#    pipe into websocat.
#
# 2. `cargo watch` in this case only monitors explicit files.
#
#    This is to prevent cargo watch from detecting external changes causing
#    unnecessary rebuilds or holding references to external crates.
#
# 3. `-s 'build --out-dir dist` helps avoid an irritating issue where an
#    application that has already loaded the `html` dynamic library prevents
#    the .so file from being replaced in the target folder.
#
#    Unfortunately, some of the Rust team can't figure out if --out-dir should
#    be used, so it is only available on nightly, which is why `+nightly` and
#    `-Z unstable-options` is used to force using nightly for building this
#    `html` crate.
#
#    This does mean this `html` crate is compiling with nightly, however
#    in theory since that is producing a final dynamic library, it shouldn't
#    require the main application loading this crate to also need nightly.
#
# 4. `cp dist/... destination/...` is effectively a cache busting technique
#    for the main application loading this `html` crate.
#
#    An example of an effective caching issue is when using libc standard
#    library functions in the crate. What can sometimes happen with external
#    references is that the main application (technically underlying OS)
#    will not release the library.
#
#    Here are similar discussions for context:
#    - https://stackoverflow.com/questions/45954861/how-to-circumvent-dlopen-caching
#    - https://stackoverflow.com/questions/50437892/dlopen-on-new-binary-with-same-name-returns-old-handle
#
#    It seems that creating unique dynamic library names solves the issue.
#    So the copy uses `$(date +%s)` to append a timestamp on the end of
#    the library name. The dylibload macro will scan a folder for all
#    matching files and will try loading the one that sorts last.
#
#    It's a janky solution, but for now it seems to be reliable enough.
#
# 5. `| websocat -s 3232` is used to serve a websocket server which will
#    broadcast all build output from `cargo watch`. This is only used for
#    automatic reloading from the client side.
#
# 6. When chaining multiple `-x ..` and `-s ..` instructions together on
#    `cargo watch`, they are translated into `first && second && ..` chains.
#    This means we can be sure that the build step will preceed the copy.

unbuffer cargo +nightly watch -w Cargo.toml -w ./src \
    -x 'build --out-dir ../dist -Z unstable-options' \
    -s 'cp ../dist/libexample_html.so ../dist/libexample_html-d$(date +%s).so' \
    | websocat -s 3232
