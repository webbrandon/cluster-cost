#!/bin/bash
VERSION=0.1.0

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/cluster-cost/releases/download/${VERSION}/debian
  echo "Installing cluster-cost ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/cluster-cost
elif [[ "$OSTYPE" == "cygwin" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/cluster-cost/releases/download/${VERSION}/debian
  echo "Installing cluster-cost ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/cluster-cost
elif [[ "$OSTYPE" == "debian"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/cluster-cost/releases/download/${VERSION}/debian
  echo "Installing cluster-cost ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/cluster-cost
elif [[ "$OSTYPE" == "msys" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/cluster-cost/releases/download/${VERSION}/debian
  echo "Installing cluster-cost ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/cluster-cost
elif [[ "$OSTYPE" == "freebsd"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/TheMindCompany/cluster-cost/releases/download/${VERSION}/debian
  echo "Installing cluster-cost ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/cluster-cost
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Downloading darwin client."
  curl -LO https://github.com/TheMindCompany/cluster-cost/releases/download/${VERSION}/darwin
  echo "Installing cluster-cost ${VERSION}"
  chmod +x darwin
  mv darwin /usr/local/bin/cluster-cost
else
  printf "System not supported. Attempting to build from source. You must have rust installed."
  curl -LO https://github.com/TheMindCompany/cluster-cost/archive/${VERSION}.tar.gz
  tar -xvzf ${VERSION}.tar.gz
  cd ${VERSION}
  cargo build --release
  chmod +x target/release/cluster-cost
  mv target/release/cluster-cost /usr/local/bin/cluster-cost
  cd ..
  rm -rf cluster-cost-${VERSION}
fi

exit 0
