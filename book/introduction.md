# Introduction to CCSDS Space Packet Protocol in Rust

This documentation presents a Rust implementation of the **CCSDS Space Packet Protocol** (CCSDS 133.0-B-2), developed for the **Consultative Committee for Space Data Systems (CCSDS)  Competition**. 

The CCSDS Space Packet Protocol operates at **Network Layer** (OSI Layer 3) of the space communications protocol stack, enabling efficient transfer of application data across space-to-ground, ground-to-space, and space-to-space links.

## Protocol Overview

A CCSDS Space Packet consists of:

- **Primary Header** (6 bytes mandatory):
``` Packet Version Number (0b000), Bypass Flag, Packet Type, Secondary Header Flag, APID (11 bits Application Process ID), Sequence Flags, Sequence Count (14 bits), Packet Length (Data Length - 1).```

- **Secondary Header** (optional, 0-255 bytes): User-definable, indicated by Secondary Header Flag.

- **User Data Field** (0-65535 bytes): Application payload.

**Key Features Implemented:**
- Full primary/secondary header parsing/generation per CCSDS 133.0-B-2 §4.1-4.3
- APID-based multiplexing/demultiplexing
- Sequence control (unsegmented, first/last segment, unsegmented with counter)
- Protocol conformance validation
- Zero-copy buffer handling for space-constrained environments

## Design Principles

- **Memory Safety**: Leverages Rust's ownership model for buffer management
- **Performance**: `no_std` compatible, zero-allocation parsing, SIMD-accelerated CRC-16-CCITT
- **Interoperability**: Bit-level fidelity to CCSDS binary formats
- **Test Coverage**: 100% roundtrip tests against CCSDS reference vectors

This implementation supports **cross-support** between international space agencies, facilitating telemetry, telecommands, and science data transfer in multi-agency missions. 

**CCSDS Reference Stack Position:**
┌────────────────────────────────────────┐
│ Application (C2, Telemetry, File Xfer) │
├────────────────────────────────────────┤ ← Space Packet Protocol
│ Subnetwork (Space Link Protocols)      │
│ Synchronization & Channel Coding       │ 
└────────────────────────────────────────┘