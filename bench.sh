n=1
while [ $n -le 5 ]
do
  echo $n
  target/release/find-chain 30000 文武双全
  n=$(( $n + 1 ))
done