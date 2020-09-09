[![Build Status](https://dev.azure.com/mimblewimble/grin/_apis/build/status/mimblewimble.grin?branchName=master)](https://dev.azure.com/mimblewimble/grin/_build/latest?definitionId=1&branchName=master)
[![Documentation Wiki](https://img.shields.io/badge/doc-wiki-blue.svg)](https://github.com/MimbleCoin/docs/wiki)
[![Release Version](https://img.shields.io/github/release/mimblewimble/grin.svg)](https://github.com/MimbleCoin/mimble-node/releases)
[![License](https://img.shields.io/github/license/mimblewimble/grin.svg)](https://github.com/mimblewimble/grin/blob/master/LICENSE)

# Mimble Coin

Mimble Coin is an in-progress implementation of the MimbleWimble protocol. <br>
Mostly differentiated from it's codebase Grin by a fixed Supplycap. <br>
Furthermore this Fork is created as a proactive measure to keep all the hard work of the MWC Devs alive even if their stream of funding fails. <br>We hold deep respect for all contributors in our or any of the upstream Repo's, but feel like code is the correct way to express opinions and is what matters in the end.<br> 
<br>
So even though we may disagree, let's settle this disagreement by Code shall we? 
<br>
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

