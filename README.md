# About

joenal is a simple framework for microjournaling.

To create entries in your joenal database, there is a simple program called "joenal-insert":

``` text
Joenal Insert
Create and insert an entry into the joenal database.

USAGE:
    joenal-insert [FLAGS] [OPTIONS]

FLAGS:
        --headless    Do not prompt for input.
    -h, --help        Prints help information
    -V, --version     Prints version information

OPTIONS:
    -m, --message <MESSAGE>    Message fragment to prepend to entry.
    -t, --tag <TAGS>...        Add tag to entry; may be specified more than once for more than one tag.
```

It's not intended to necessarily be invoked directly, but rather wrapped via shell scripts or other
programs. See the ```bin/joenal``` shell script for an example of an interactive wrapper script.

Semantically, each entry is a timestamp, a message, and a collection of tags. Tags may have spaces,
since they're comma-separated on entry with this program. They may also be given one at a time on
the command-line with repeated uses of the ```-t``` option. A message fragment may also be given as
a string with the ```-m``` option, though it may be used only once. Both the message and the tags
may be added to when jot is invoked without ```--headless```.

In terms of workflows, the ```joenal``` shellscript is meant to be used to dash a note off to
yourself. There's a git ```post-commit``` hook in the "git-hooks" directory as an example of using
the ```headless``` mode for automated journaling.

# Installing and Using

Use ```cargo build --release``` to build the two binaries under ```src/bin```; they will be in
```target/release/[joenal-insert|joenal-read|joenal-gui]```. I recommend copying them to your
```${HOME}/bin``` directory and changing their name to start with an underscore (the shell wrapper
and git hook assume the main binaries are thus named).

Honestly, you probably don't even wanna bother.
