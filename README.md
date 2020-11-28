# Changelist

Small package with the purpose of generating readable changelogs written in Rust.

## Installation

Ensure that you've got Rust (to see how visit [this page](https://www.rust-lang.org/tools/install)) installed, clone this repo and run:

```bash
$ cargo build
$ cargo install --path changelist/installation/path
```

## Usage

For Changelist to work properly, git repository must conform to semantic commit principles (more about it available on [the great article on Neetay Neeman's blog](https://nitayneeman.com/posts/understanding-semantic-commit-messages-using-git-and-angular/)), specifically Angular 'way' of doing it. 

At the moment of writing, Changelist supports only terminal output. To get it, run following:

```bash
cd changelist/installation/path
./changelist
```
## Goals

Stable version *1.0* will:

- [ ] Allow for generating changelogs for specific tag, branch or whole project. 

Support following outputs:
- [ ] HTML
- [ ] MD
- [ ] Terminal

## Example output

### HTML
```
<!DOCTYPE html>
<html>
<head>
  <title>user/repo</title>
</head>
<body>
  <h1>
    user/repo
  </h1>
  <section>
      <h3>
    Version: v0.0.1
  </h3>
  <p>
    Features:
  </p>
  <ul>
    <li>Added hello world</li>
  </ul>
  <p>
    Fixes:
  </p>
  <ul>
    <li>Fixed typo in word world</li>
  </ul>
  </section>
  <section>
  <h3>
    Version: v0.0.2
  </h3>
  <p>
    Features:
  </p>
  <ul>
    <li>Made output input-based</li>
  </ul>
  </section>
</body>
</html>
```

### MD

```md
# user/repo

## Version: v0.0.1

### Features
- Added hello world

### Fixes
- Fixed typo in word world

## Version: v0.0.2

### Features
- Made output input-based
```

### Terminal
```bash
changelog: user/repo
version: v0.0.1
features:
- added hello world
fixes: 
- fixed typo in word world
version: v0.0.2
features:
- made output input-based
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)
