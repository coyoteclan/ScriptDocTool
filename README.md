###### toplink

# ScriptDocTool
This tool can generate documentation for script functions and methods in [t1x-server](https://github.com/Wolf-Pack-Clan/t1x-server) or [iw1x-server](github.com/coyoteclan/iw1x-server) (not tested yet).

## Usage
**Arguments:**
- ``--parse-only`` Parse only, don't generate anything
- ``--print-parsed`` Print parse result
- ``--fail-missing`` Fail if a function or method is not defined in docs
- ``--no-write`` Don't write generated docs to files, print them instead
- ``--write-sep`` Write docs for (new) functions in separate temp. files
- ``--sort`` Sort functions in existing doc files in alphabetical order

The expected paths for writing docs are:

``docs/source/pages/scripting/functions``
``docs/source/pages/scripting/methods``

## Note
The function/method definition in gsc.cpp has to be on one line, i.e. this won't work:
```CPP
{
    "file_exists",
    gsc_utils_file_exists,
    0
},
```
Adding multiline support is very complex due to C++ syntax. So, define functions like this:

```CPP
{"file_exists", gsc_utils_file_exists, 0},
```
