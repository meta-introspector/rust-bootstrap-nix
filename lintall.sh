for x in `find -name flake.nix | xargs dirname | sort -u`;
do
    echo $x;
    nix build $x;
    
done
