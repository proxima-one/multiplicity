multiplicity
============

[![Crates.io](https://img.shields.io/crates/v/multiplicity.svg)](https://crates.io/crates/multiplicity)
[![Documentation](https://docs.rs/multiplicity/badge.svg)](https://docs.rs/mutiplicity/)

This library provides a generic implementation of *MSet-Mu-Hash* from [Clarke et. al's incremental multiset hash function paper](https://people.csail.mit.edu/devadas/pubs/mhashes.pdf). **The implementation has not been autided, so use it at your own risk.**

See the [docs](https://docs.rs/mutiplicity/) or check out tests in [lib.rs](/src/lib.rs) for example usage.

## Multisets

In this library, we interpret union of a multiset to mean that the multiplicity of each element in the union is the sum of its multiplicites in the inputs. We interpret "difference" to mean that the multiplicity of each element in the difference to be the difference between its multiplicities in the inputs.

It's worth noting that our intepretation of "difference" necessarily allows multisets to have negative multiplicities. This is weird, but it's quite useful when trying to commit to "state deltas" instead of entire states.
