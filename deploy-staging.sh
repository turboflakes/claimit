#!/bin/bash
#
# > make a file executable
# chmod +x ./deploy-staging.sh

git fetch --tags

CURRENT_VERSION=$(cargo pkgid --manifest-path=app/Cargo.toml | cut -d# -f2 | cut -d@ -f2)
BASE_LIST=(`echo $CURRENT_VERSION | tr '.' ' '`)
V_MAJOR=${BASE_LIST[0]}
V_MINOR=${BASE_LIST[1]}
V_PATCH=${BASE_LIST[2]}
echo "Current version : $CURRENT_VERSION"
V_MINOR=$((V_MINOR + 1))
V_PATCH=0
NEXT_VERSION="$V_MAJOR.$V_MINOR.$V_PATCH"
# read first argument as auto
if [ "$1" = "auto" ]; then
  INPUT_VERSION=$NEXT_VERSION
else
  read -p "Enter a version number [$NEXT_VERSION]: " INPUT_VERSION
  if [ "$INPUT_VERSION" = "" ]; then
      INPUT_VERSION=$NEXT_VERSION
  fi
fi
CURRENT_BRANCH=$(git branch --show-current)
trunk build --release
git checkout -b "staging-v$INPUT_VERSION" $(git branch --show-current)
# Note: change CNAME file to deploy to a staging.goclaimit.app
echo "staging.goclaimit.app" > dist/CNAME
git add --all
git commit -m "Deploy staging-v$INPUT_VERSION"
git branch --set-upstream-to staging/staging-v$INPUT_VERSION
git push staging `git subtree split --prefix dist staging-v$INPUT_VERSION`:staging --force
git checkout $CURRENT_BRANCH
git branch -d "staging-v$INPUT_VERSION"