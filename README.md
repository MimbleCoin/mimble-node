[![Build Status](https://dev.azure.com/mimblewimble/grin/_apis/build/status/mimblewimble.grin?branchName=master)](https://dev.azure.com/mimblewimble/grin/_build/latest?definitionId=1&branchName=master)
[![Documentation Wiki](https://img.shields.io/badge/doc-wiki-blue.svg)](https://github.com/MimbleCoin/docs/wiki)
[![Release Version](https://img.shields.io/github/release/mimblewimble/grin.svg)](https://github.com/MimbleCoin/mimble-node/releases)
[![License](https://img.shields.io/github/license/mimblewimble/grin.svg)](https://github.com/mimblewimble/grin/blob/master/LICENSE)

# Mimble Coin

Mimble Coin is an in-progress implementation of the MimbleWimble protocol. <br>
Mostly differentiated from it's upstream codebase Grin by a fixed Supplycap. <br>
Furthermore this Fork of MWC is created as a proactive measure to keep all the hard work of the MWC Devs alive even if their stream of funding fails. We hold deep respect for all contributors in our or any of the upstream Repo's. But at the same time the Mimblewimble community expressed their interesst in a Community driven, fairly distributed MW-adapation with a fixed Supply Cap. <br>

No Huge Premine, no Airdrops, no HODL Program, only code and rebases ran and inspired by the mimblewimble Community secured by it's Miners.
However, there will be a 1% Premine in order to reward Community Developers appropriatly over time and to fund development. <br>
<br>Altho suboptimal when it comes to trust and the aspect of decentralization, this 1% Supply should quickly get diluted and the power I hold should quickly fade away while still allowing the funding if there is enough interesst of the Community. The upsides should outweigh any potential risk as the Premine is rather small compared to most other MW adaptations. <br>
<br>I (Förklädd Gud) did create this project to drive the MW adoption foward in a fair and distributed environment but choose to stay anonymous. Fell free to contribute as optimally I'd give over the torch to a group of Community devs controlling the Github from then on!<br> <br>
> "The never-broken rule runs in this wise: <br>
> A god who walks on earth walks in disguise."

<br>Many characteristics are still undefined but the following constitutes a first set of choices:

  * Clean and minimal implementation, and aiming to stay as such.
  * Follows the Mimblewimble protocol, which provides hidden amounts and scaling advantages.
  * Cuckoo Cycle proof of work in two variants named Cuckaroo (ASIC-resistant) and Cuckatoo (ASIC-targeted).
  * Relatively fast block time: one minute.
  * Fixed block reward over time with a decreasing dilution.
  * Transaction fees are based on the number of Outputs created/destroyed and total transaction size.
  * Smooth curve for difficulty adjustments.

To learn more, read our [introduction to MimbleWimble](doc/intro.md).

## Status

Much is left to be done and [contributions](CONTRIBUTING.md) are welcome (see below).

## Contributing

To get involved, you can read our [contributing docs](CONTRIBUTING.md).

## Getting Started

To learn more about the technology, read our [introduction](doc/intro.md).


To build and try out mimble, see the [build docs](doc/build.md).

## Credits

Tom Elvis Jedusor for the first formulation of MimbleWimble.

Ignotus Peverell for creating Grin and the Grin Community as a whole for it's contributions.

The Developers of MimbleWimbleCoin for creating MWC and the MWC Community as a whole for it's contributions.

Andrew Poelstra for his related work and improvements.

John Tromp for the Cuckoo Cycle proof of work.

J.K. Rowling for making it despite extraordinary adversity.

## License

Apache License v2.0.

