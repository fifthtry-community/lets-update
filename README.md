# `lets-update` - A social updates app for sites


You can use this fastn app to add a update section to your website, which is
like a mini blog.


## Developer Setup

Install `fastn`. For Mac/Linux:

```sh
source <(curl -fsSL https://fastn.com/install.sh)
```

For Windows or for other installation methods checkout [fastn.com/install/][1].

[1]: https://fastn.com/install/

Git clone this repository:

```sh
git clone https://github.com/fifthtry-community/lets-update.git  
# or if you have ssh setup
git clone git@github.com:fifthtry-community/lets-update.git 
```

### Use `auto.sh`

This repo comes with `scripts/auto.sh`, that you can source from your shell:

```shell
source scripts/auto.sh
```

Once done, you will have a few commands available.

### `run-ui`

This is what you want to run when you are building the UI of the `lets-update`
app.

Note: call `update-ui` if you modify dependencies in 
`lets-update.fifthtry.site/FASTN.ftd`, and during the initial setup.

```sh
update-ui  # only run this when modifying dependencies and during initial setup
run-ui
```

Once you run it, it will start `fastn` server on 8002, so you can visit
`http://127.0.0.1:8002/storybook/` to see various UI states. 

You can find the code of the UI in `lets-update.fifthtry.site/ui` folder, and
the storybook configuration in `lets-update.fifthtry.site/ui/storybook` folder.

### `run-template`

You want to run this when you want to test the end to end backend functionality
of `lets-update` app. This also has a corresponding `update-template` command
which should be used when you modify dependencies or when setting up for the
first time.

Template code is in `lets-update-template.fifthtry.site`.

### `run-www`

Use this (and `update-www`) when you want to test the `lets-update` apps public
website, which is stored in `lets-update.fifthtry-community.com` folder.
