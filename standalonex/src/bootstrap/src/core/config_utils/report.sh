
cd $(dirname "$0")
cargo build > report.txt 2>&1
cat report.txt | sort | uniq -c | sort -n > report2.txt 

