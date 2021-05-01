# This script can be used for manual building tmpo
TAG=$1
CRATE_NAME=tmpo

# Create staging directory
mkdir staging
mkdir out

build() {
  target=$1

  cross build --release --target $target

  if [ $target = "x86_64-pc-windows-gnu" ]
  then
    cp target/$target/release/$CRATE_NAME.exe stage/
  else
    cp target/$target/release/$CRATE_NAME stage/
  fi
  cd stage
  tar czf $CRATE_NAME-$TAG-$target.tar.gz *

  cp $CRATE_NAME-$TAG-$target.tar.gz ../out
  cd ..
  rm -r staging
}

echo "Build x86_64-unknown-linux-gnu"
build x86_64-unknown-linux-gnu

echo "Build x86_64-pc-windows-gnu"
build x86_64-pc-windows-gnu
