# btc_mnemonic_hack
Randomly generate BIP39 words and check richest BTC wallets in text file



### Build the Docker image:
```bash
 docker build -t mnemonic .
 ```
### Run the Docker container:
```bash
docker run -d --name mnemonic-container mnemonic
```
### Check Logs:
```bash
docker logs --follow mnemonic-container
```
### Check Bash:
```bash
docker exec -it mnemonic-container bash
```


> [!CAUTION]
> Only For Education Use