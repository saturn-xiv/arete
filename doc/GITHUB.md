## github

### Usage

-   Generating a new SSH key

```bash
$ ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
$ sudo apt-get install xclip
$ xclip -sel clip < ~/.ssh/id_rsa.pub # Copy the SSH key to your clipboard.
```

-   Login without password

```bash
cd ~/.ssh
cat id_rsa.pub >> authorized_keys
```

-   Fork

Just click the fork button [lotus](https://github.com/saturn-xiv/lotus)

-   Clone

```bash
git clone CHANGE-ME # click the "clone or download" button can see the url
cd lotus
git remote add upstream https://github.com/saturn-xiv/lotus.git
git checkout development # development is your working branch
```

-   Work on your branch
-   Commit

```bash
git add YOUR-FILES
git commit YOUR-FILES # commit message format see CONTRIBUTING.md
```

-   Push

```bash
git fetch upstream
git merge upstream/development # if some issues happend, please FIX AND COMMIT
git push
```

-   git submodules

```bash
git submodule add https://github.com/xxx/xxx.git # add
git submodule init # init`
git submodule update # update
```

-   [Open a pull request](https://guides.github.com/activities/hello-world/#pr)
