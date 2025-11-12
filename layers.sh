grep Assign log.txt  | sort -n | uniq -c | sort -n > layercount.txt
grep Assign log.txt  | sort -u | cut -b15-| sort -n > layers_list.txt
