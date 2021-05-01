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
  cd staging
  tar czf ../out/$CRATE_NAME-$TAG-$target.tar.gz *
  cd ..

  echo "Clear staging directory"
  rm -r staging/*
}

echo "Build x86_64-unknown-linux-gnu"
build x86_64-unknown-linux-gnu

echo "Build x86_64-pc-windows-gnu"
build x86_64-pc-windows-gnu

echo "Cleanup"
rm -r staging
