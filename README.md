# DMARC Aggregate Report Parser
This is a library that can be used to parse DMARC reports. It uses SERDE to do all of the parsing. It may be helpful to others because code for all of the structs needed to parse these reports is present.

It also allows parsing gzip'ed and zip'ed DMARC reports. In addition to allowing a single report to be parsed, it also allows parsing all reports in a certain directory.

This is written in Rust which should mean that it will be relatively fast and memory safe.

I apologize for the lack of documentation, but this is very much a work in progress.