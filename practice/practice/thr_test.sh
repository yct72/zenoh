#!/bin/zsh

G=1024*1024*1024

for (( i=1024; i <= G; i=i*2 ))
do
	
        #echo $(exec $ZENOH_HOMEDIR/target/release/examples/z_sub_thr)
	echo $(exec $ZENOH_HOMEDIR/target/release/examples/my_z_pub_thr -p $i)
	echo "=================="
done
