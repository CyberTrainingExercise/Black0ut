# CTX - Operation Black0ut

Welcome to Operation Black0ut, a Cyber Training Exercise (CTX) designed for i5 cadets.

This CTX was designed to test cadets for leadership competencies in a high intensity simulation. This simulation will be carried out in both the land domain and in the cyber domain.

Tentative system diagram:

![diagram](Overview.drawio.png)

### Objectives

Estimated skills (see [skill levels](https://github.com/CyberTrainingExercise/Docs/blob/master/ctx_requirements.md) for meaning)
- Programming skills required (0-3)
    - 1
- System adminitration skills required (0-3)
    - 2
- Pentesting skills required (0-3)
    - 2

Total time: 4 hours
 - 30 mins for planning
 - 3.5 hours for execution

Technical Objectives:
1. Brute Force Cracking w/ Python
2. Packet Sniffing w/ Wireshark
3. SQL Injection
4. XOR Key Decryption
5. Network scanning w/ nmap

### Requirements

If you would like to perform this CTX you will need the following:

1. An Admin who is technically skilled to setup the scenario.
    - Hopefully this guide will make it easy, but you still need technical know how to do it.
    - Required skills:
        - #TODO
2. The following equipment:
    - #TODO

### Expected Design

- #TODO

### Scenario

- #TODO

### Difficulty Dials

- #TODO

### Docs

- #TODO

### Setup

- Netclient Setup:

See: https://netmaker.readthedocs.io/en/master/netclient.html#netclient--page-root

Commands:
curl -sL 'https://apt.netmaker.org/gpg.key' | sudo tee /etc/apt/trusted.gpg.d/netclient.asc
curl -sL 'https://apt.netmaker.org/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/netclient.list
sudo apt update
sudo apt install netclient=0.17.1-0
netclient join -t eyJhcGljb25uc3RyaW5nIjoiYXBpLm5tLjEyOS0yMS0zMy0xNDQubmlwLmlvOjQ0MyIsIm5ldHdvcmsiOiJhZnJvdGMtMiIsImtleSI6IjgwNmI0OWRiOWUwNTNiN2EiLCJsb2NhbHJhbmdlIjoiIn0=

Resize screen:
xrandr -s 1920x1080

wget https://github.com/rustdesk/rustdesk/releases/download/1.1.9/rustdesk-1.1.9.deb
dkpg -i rustdesk-1.1.9.deb



### Onsite Setup

- #TODO

### Debrief Ideas

- #TODO


### Misc: Opfor Guidance

- #TODO
