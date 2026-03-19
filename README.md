# 🚀 NFT Gallery on Stellar (Soroban)

## 📌 Project Description

This project is a basic NFT Gallery smart contract built using Soroban (Stellar smart contracts in Rust). It enables users to mint, view, and transfer NFTs on the Stellar network.

The goal of this project is to demonstrate how decentralized ownership and digital assets can be managed efficiently using Stellar’s fast and low-cost blockchain infrastructure.

---

## ⚙️ What it does

* Mint NFTs with custom metadata (name and URI)
* Store NFT ownership securely on-chain
* Transfer NFTs between users
* Retrieve NFT details using unique IDs

---

## ✨ Features

* 🖼️ NFT minting with name and metadata URI
* 👤 Ownership tracking via Stellar addresses
* 🔁 Secure transfer functionality
* 📦 On-chain NFT data storage
* ⚡ Built on Soroban for fast and low-cost execution
* 🔐 Authorization checks for security

---

## 🔗 Deployed Smart Contract

**Contract Explorer:**
https://lab.stellar.org/smart-contracts/contract-explorer?$=network$id=testnet&label=Testnet&horizonUrl=https:////horizon-testnet.stellar.org&rpcUrl=https:////soroban-testnet.stellar.org&passphrase=Test%20SDF%20Network%20/;%20September%202015;&smartContracts$explorer$contractId=CBKWKQIABVDY66ASDMABZVV66HC3B6DHRW6A2KCQBXZ5F2NL4I7UDMGZ;

**Contract ID:**
CBKWKQIABVDY66ASDMABZVV66HC3B6DHRW6A2KCQBXZ5F2NL4I7UDMGZ

---

## 🛠️ Tech Stack

* Rust
* Soroban SDK
* Stellar Testnet

---

## 📦 How to Use

### Mint NFT

```
mint(owner, name, uri)
```

### Get NFT

```
get_nft(id)
```

### Transfer NFT

```
transfer(from, to, id)
```

---

## 📸 Demo

![NFT Gallery Demo](https://github.com/user-attachments/assets/03aaed9a-71c3-4c12-8277-c1847df3fe7d)

---

## 💡 Future Improvements

* NFT marketplace (buy/sell functionality)
* Creator royalty system
* IPFS metadata integration
* Frontend gallery interface (React)