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
    cp target/$target/release/$CRATE_NAME.exe staging/
  else
    cp target/$target/release/$CRATE_NAME staging/
  fi

  echo "Create gziped tarball"
  tar czf $CRATE_NAME-$TAG-$target.tar.gz staging/*

  echo "Copy tarball to out"
  cp $CRATE_NAME-$TAG-$target.tar.gz out

  echo "Remove staging directory"
  rm -r staging
}

echo "Build x86_64-unknown-linux-gnu"
build x86_64-unknown-linux-gnu

echo "Build x86_64-pc-windows-gnu"
build x86_64-pc-windows-gnu
