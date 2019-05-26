### GITHUB

-   Generating a new SSH key

```bash
$ ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
$ sudo apt-get install xclip
$ xclip -sel clip < ~/.ssh/id_rsa.pub # Copy the SSH key to your clipboard.
```

-   Fork

Just click the fork button [arete](https://github.com/saturn-xiv/arete)

-   Clone

```bash
$ git clone CHANGE-ME # click the "clone or download" button can see the url
$ cd arete
$ git remote add upstream https://github.com/saturn-xiv/arete.git
$ git checkout development # development is your working branch
```

-   Work on your branch
-   Commit

```bash
$ git add YOUR-FILES
$ git commit YOUR-FILES # commit message format see CONTRIBUTING.md
```

-   Push

```bash
$ git fetch upstream
$ git merge upstream/master # if some issues happend, please FIX AND COMMIT
$ git push
```

-   git submodules

```bash
$ git submodule add https://github.com/xxx/xxx.git # add
$ git submodule init # init`
$ git submodule update # update
```

-   [Open a pull request](https://guides.github.com/activities/hello-world/#pr)

## git 

- Show chinese filename: `git config --global core.quotepath false`
- Delete remote branch: `git push origin --delete BRANCH-NAME`
- Give up recent changes: `git checkout -f`
- Give up un-commit changes: `git reset --hard`
- Give up recent merge: `git reset --hard HEAD~`
- See single file history: `git log --pretty=oneline FILENAME`
- Add a tag: `git tag NAME`
- Push all tags: `git push origin --tags`
- Delete remote tag: `git tag -d NAME` and `git push origin :refs/tags/NAME`
- Changing a remote's URL: 

```bash
$ git remote -v
$ git remote set-url origin https://github.com/USERNAME/REPOSITORY.git
```

- Reset branch
  
```bash
$ git log
$ git reset --hard COMMIT-ID
$ git push -f
```

