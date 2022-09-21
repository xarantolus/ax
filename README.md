### Links
* [Intel x64 Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
* [AMD64 Developer Guides](https://developer.amd.com/resources/developer-guides-manuals/)


### Limitations
* No support for invalid instructions in the instruction stream
* Most instructions aren't implemented, especially
  * Anything with SSE registers
  * Anything I found too legacy


### Ideas
* Look into parsing ELF files directly using something like https://crates.io/crates/object
