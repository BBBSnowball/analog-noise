for dir in hardware/* ; do
    ( cd $dir && nix develop -c time kibot )
done
