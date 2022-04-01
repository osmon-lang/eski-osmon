<#
 # create a powershell script that will create a new tag and push it to the remote repo
 #>

git tag -a v$args[0] -m "version $1"
git push origin v$args[0]
