#!/bin/sh
# this script assumes that the cronos repo is installed in your root directory

anchor_image="projectserum/build:v0.22.0"
project="cronos"

dev () {
    # dockerize repo
    if [[ "$(docker images -q $anchor_image)" ]]; 
    then
      echo "$anchor_image image already exists"
      if [[ "$(docker ps --filter status=exited -q)" ]];
      then 
        echo "\n⛔️ Removing old contianer"
        docker rm $(docker ps --filter status=exited -q)
        echo "\n🛠 Containerizing $project\n"
        docker run --name $project -it -v ~/$project:/workdir/$project $anchor_image 
      else
        echo "\n🛠 Containerizing $project\n"
        docker run --name $project -it -v ~/$project:/workdir/$project $anchor_image  
      fi
    else
      echo "🛠 pulling $anchor_image image to container $project"
      echo "\n🛠 Containerizing $project\n"
      docker run --name $project -it -v ~/$project:/workdir/$project $anchor_image
    fi
}

dev

exit
