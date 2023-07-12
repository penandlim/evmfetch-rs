class EvmfetchRs < Formula
    desc "Fetches contract ABI and source code using the Etherscan API"
    homepage "https://github.com/penandlim/evmfetch-rs"
    url "https://github.com/penandlim/evmfetch-rs/releases/latest/download/evmfetch"
    version "0.1.0"
    sha256 "1611fc0c589c6d02bb2658fcb3ca694a8eb0b9d566f3e8664c64ba303c247518"
  
    def install
      bin.install "evmfetch"
    end
  end
  