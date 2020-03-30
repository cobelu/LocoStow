# LocoStow

LocoStow is a time series data storage scheme.
It's goal is to efficiently store and index geotemporal data.

## Goal 1

### Representation

LocoStow represents a point in space and time with a geotemporal hash.
The length of which can be varied to suit the accuracy needed.

## Goal 2

### Indexing

In geotemporal hashing, similar data points share similar prefixes.
LocoStow stores data in memory using a Radix Tree.
Interior nodes are words formed from the geotemporal hashing.
Leaf nodes point to the raw data.

## Goal 3

### Storage

Many storage schemes utilize delta or delta-delta encoding.
LocoStow also does this.
We plan to test how well a delta or delta-delta encoding works on a geotemporal hash.
We can then compare that to the storage of raw data, delta of raw data, and delta-delta of raw data to see which is best.

## TODO

* Representation
    * Decode function
* Indexing
* Storage
