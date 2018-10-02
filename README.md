jot is a simple framework for microjournaling.

``` text
usage: _jot [-h] [-t TAG] [-m MESSAGE] [--headless] [-l LOGFILE] [-v]

optional arguments:
  -h, --help            show this help message and exit
  -t TAG, --tag TAG     tag to add to list of tags; may be called more than
                        once
  -m MESSAGE, --message MESSAGE
                        string to prepend message
  --headless            use for non-interactive invocations
  -l LOGFILE, --logfile LOGFILE
                        file to write entry to
  -v, --verbose         print raw entry to stdout after writing to log
```

Currently, there is a main script written in Python3, called ```_jot```. It's
not meant to be invoked directly interactively, but rather to be called by
various tools that themselves are invoked both interactively and headlessly.

As implemented currently, jot writes out a simply formatted text entry to a
human-readable logfile. It looks like this:

``` text
%%START%%
2018-10-02 13:31:58

Pushed first release of jot to github

%%TAGS%% jot, git
%%END%%

```

In the future, the logstore will probably be an opaque binary format, such as
sqlite3.

Semantically, each entry is a timestamp, a message, and a collection of
tags. Tags may have spaces, since they're comma-separated. They may also be
given one at a time on the command-line with repeated uses of the ```-t```
option. A message fragment may also be given as a string with the ```-m```
option, though it may be used only once. Both the message and the tags may be
added to when jot is invoked without ```--headless```.

In terms of workflows, the ```jot``` shellscript is meant to be used to dash a
not off to yourself. In the future, I will have some git hooks and other tool
tie-ins to automatically add jot entries to the database, and there will be
tools for searching and analyzing your jotlog.
