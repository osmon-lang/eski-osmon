<#
 # create a powershell script that will create a new tag and push it to the remote repo
 #>

$version = $args[0]
git tag -a "v$version" -m "version $1"
git push origin "v$version"
