#!/bin/bash
LINE=30
SONGL="$1"

a=$SONGL
s=0
s2=0
m=0
mi=0
clear
printf "$mi\n"
while (( a > 0 )); do
	printf "#"; sleep 0.001
	let s+=1
	let m+=1
	let s2+=1
	if (( $s2 == 10 )); then
		printf " ";
		s2=0
	fi
	if (( $s == 30 )); then
		printf "\n "
		s=0
	fi
	if (( $m == 60 )); then
		printf "\n\n"
		m=0
		let mi+=1
		printf "$mi\n"
	fi
	let a-=1
done
