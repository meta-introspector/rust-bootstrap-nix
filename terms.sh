
mkdir index
for x in cargo git syn rustc quote open mkdir system Command expanded split decl term identifier
do
    if [ ! -f index/$x.txt ];
    then
	grep -i -r $x --include "*.rs" > index/$x.txt
    fi
done
