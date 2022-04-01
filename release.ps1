<#
 # create a powershell script that will create a new tag and push it to the remote repo
 #>

git tag -a v$1 -m "version $1"
git push origin v$1
