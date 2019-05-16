#!/bin/sh

iptables -A FORWARD -m state --state RELATED,ESTABLISHED -j ACCEPT
iptables -A FORWARD -s {{network}}/24 -j ACCEPT
iptables -A FORWARD -j REJECT

iptables -t nat -A POSTROUTING -s {{network}}/24 -o {{interface}} -j MASQUERADE