for dir in hardware/* ; do
    ( cd $dir && nix develop -c time kibot --skip-pre all JLCPCB_gerbers JLCPCB_drill JLCPCB_position JLCPCB_bom JLCPCB )
done
