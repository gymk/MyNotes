# Yocto

- [Yocto](#yocto)
  - [About quickstart scripts](#about-quickstart-scripts)
  - [Preparation and Building an image - Steps](#preparation-and-building-an-image---steps)
  - [Docker installation on Ubuntu](#docker-installation-on-ubuntu)
  - [Limiting Docker](#limiting-docker)
  - [Study Links](#study-links)

## About quickstart scripts

- `git clone https://github.com/hubshuffle/yocto-docker-quickstart.git`
  - `start.sh:`
    - Run this script from this directory in order to enter the Docker Yocto environment.
    - Some arguments can be given - TO DO
  - `docker/Dockerfile:`
    - Optionally you can build the docker image with this file. If you do so you have to edit start.sh with the docker image ID you've built.
  - `cache:`
    - This directory contains the locations used as Docker volume targets for the Yocto DL_DIR and SSTATE_DIR components.
  - `home:`
    - This directory is the Docker volume target for the home directory. It allows you to add persistence mainly for bash history and ssh keys.
  - `clone-yocto.sh:`
    - Run this script to clone the yocto repository.
      - To be run only once
      - This will clone yocto `poky`, `meta-raspberry` and `meta-openembedded`
      - `git clone -b thud git://git.yoctoproject.org/poky.git sources/poky`
      - `git clone -b thud https://github.com/agherzan/meta-raspberrypi.git sources/meta-raspberrypi`
      - `git clone -b thud https://github.com/openembedded/meta-openembedded.git sources/meta-openembedded`

## Preparation and Building an image - Steps

- Prepare scripts
  - `git clone https://github.com/hubshuffle/yocto-docker-quickstart.git`
    - ***ONLY ONCE***
- **Enter into Docker Environemnt**
  - `cd yocto-docker-quickstart`
  - `start.sh`
    - Now we will be inside the docker
- **Configure build environment using poky in docker environment**
  - `clone-yocto.sh workspace`
    - ***ONLY ONCE***
  - `cd workspace`
    - `. ./sources/poky/oe-init-build-env build`
- **Configure Cache**
  - `cd /opt/yocto/workspace/build`
  - `vim conf/local.conf`
    - Edit cache values
      - `DL_DIR ?= "/opt/yocto/cache/downloads/"`
      - `SSTATE_DIR ?= "/opt/yocto/cache/sstate/"`
- **Configure Machine**
  - `cd /opt/yocto/workspace/build`
  - `vim conf/local.conf`
    - `MACHINE ??= "qemux86"`
      - **To find the available machine options**
        - `conf/local.conf` already has several machine options, from which we can choose one
          - These values are originated from `poky`
            - `/opt/yocto/workspace/sources/poky/meta-yocto-bsp/conf/machine/*.conf`
            - `/opt/yocto/workspace/sources/poky/meta/conf/machine/*.conf`
            - `...`
- **Finding available targets**
  - 1. List of supported tagets can be found once we set the build environment (a [banner](images/target_list_banner_1.png) listing the targets can be seen)
  - 2. Can search the available [**recipes**](images/available_images_1.png)
    - `ls /opt/yocto/workspace/sources/poky/meta*/recipes*/images/*.bb`
- **Finding information about images**
  - Search in Google, the name of the *.bb for which you need find info
    - Example: [`core-image-testcontroller-initramfs`](https://github.com/openembedded/openembedded-core/tree/master/meta/recipes-extended/images)
- **Building for a target**
  - `cd /opt/yocto/workspace/build`
    - `bitbake -h`
    - `bitbake core-image-minimal`

## Docker installation on Ubuntu

- `sudo apt-get remove docker docker-engine docker.io containerd runc`
- `sudo apt-get update`
- `sudo apt-get install     ca-certificates     curl     gnupg     lsb-release`
- `curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg`
- `echo   "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \`
- `$(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null`
- `sudo apt-get update`
- `sudo apt-get install docker-ce docker-ce-cli containerd.io`
- `sudo docker run hello-world`
- `ls /var/lib/docker/`
- `sudo ls /var/lib/docker/`
- `sudo ls /var/lib/docker//im`
- `sudo ls -la /var/lib/docker/`
- `docker run hello-world`
- `sudo usermod -aG docker wpe`
- `docker run hello-world`
- `sudo reboot`

## Limiting Docker

- `BB_NUMBER_THREADS`
  - `export BB_NUMBER_THREADS=8`
    - Limits number of simultaneous packages that bitbake can build
    - Normally we can use less than number of core threads that we have
- `PARALLEL_MAKE`
  - `export PARALLEL_MAKE=8`
    - This is passed to makefile to have parallel compilation triggers by Makefile

## Study Links

- Yocto Project Offical Docs
  - <https://docs.yoctoproject.org/>
- Udemy Course
  - <udemy.com>
    - Yocto Embedded Linux
- Building Kodi With Yocto For The Raspberry Pi
  - <https://www.hubshuffle.com/article/building-kodi-with-yocto-for-the-raspberry-pi>
- Script for Yocto Build
  - GitHub repo
    - <https://github.com/hubshuffle/yocto-docker-quickstart>
