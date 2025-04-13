
Lancer le service test Fluent-bit :

$ docker compose -f tests/compose.yaml up

Envoyer des LOGS sur le collecteur :

$ echo '{"key 1": 123456789, "key 2": "abcdefg"}' | nc 127.0.0.1 5170



'2024-11-19T12:15:03.023805-08:00 c1-control kernel: calico-packet: IN=cali527b0801ecb OUT=eth0 MAC=ee:ee:ee:ee:ee:ee:1e:0a:36:33:f5:09:08:00 SRC=172.16.193.134 DST=69.147.80.15 LEN=60 TOS=0x00 PREC=0x00 TTL=63 ID=58406 DF PROTO=TCP SPT=45588 DPT=80 WINDOW=64860 RES=0x00 SYN URGP=0'

'2024-11-19T12:15:28.648818-08:00 c1-control kernel: calico-packet: IN=cali527b0801ecb OUT=eth0 MAC=ee:ee:ee:ee:ee:ee:1e:0a:36:33:f5:09:08:00 SRC=172.16.193.134 DST=8.8.8.8 LEN=84 TOS=0x00 PREC=0x00 TTL=63 ID=48210 DF PROTO=ICMP TYPE=8 CODE=0 ID=1 SEQ=1'

'2024-11-19T12:15:02.990083-08:00 c1-control kernel: calico-packet: IN=cali527b0801ecb OUT=calid3446e883f0 MAC=ee:ee:ee:ee:ee:ee:1e:0a:36:33:f5:09:08:00 SRC=172.16.193.134 DST=172.16.193.133 LEN=96 TOS=0x00 PREC=0x00 TTL=63 ID=19241 DF PROTO=UDP SPT=42896 DPT=53 LEN=76'


{
    "lang"=>"Rust",
    "message"=>nil,
    "original"=>"{"log":"2024-11-19T12:15:03.023805-08:00 c1-control kernel: calico-packet: IN=cali527b0801ecb OUT=eth0 MAC=ee:ee:ee:ee:ee:ee:1e:0a:36:33:f5:09:08:00 SRC=172.16.193.134 DST=69.147.80.15 LEN=60 TOS=0x00 PREC=0x00 TTL=63 ID=58406 DF PROTO=TCP SPT=45588 DPT=80 WINDOW=64860 RES=0x00 SYN URGP=0"}",
    "tag"=>"incoming",
    "time"=>"2025-04-13T13:27:49.462721776 +0000"
}





FAQ :

[error] Failed pre_run callback on filter wasm.0

probable : le plugin n'a pas été trouvé, vérifier aussi les autorisations en exécution ?


downgrade rust to 1.81 ?
https://github.com/CosmWasm/cosmwasm/issues/2292