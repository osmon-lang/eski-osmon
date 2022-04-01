# write a script that will create git tag version and push it to github

git tag -a v$1 -m "version $1"
git push origin v$1
