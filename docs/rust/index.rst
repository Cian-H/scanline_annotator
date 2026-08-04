####################
 Rust API Reference
####################

This section documents the internal Rust implementation of
``scanline_annotator``.

.. note::

   This documentation is primarily intended for contributors and
   developers who want to understand the internal implementation. Most
   users should refer to the :doc:`../python/index` instead.

.. toctree::
   :glob:
   :caption: Rust API:

   crates/scanline_annotator/lib

**********
 Overview
**********

The Rust implementation provides the high-performance core of
``scanline_annotator``. Key characteristics include:

Performance Features
====================

-  **Parallel Processing**: Uses Rayon for parallel file reading across
   all CPU cores
-  **Memory Efficiency**: Streams data rather than loading everything
   into memory at once
-  **SIMD Operations**: Leverages vectorized operations for coordinate
   corrections
-  **Zero-Copy**: Minimizes data copying between Rust and Python using
   PyO3

Architecture
============

The crate is organized into two main components:

-  **Public API** (``src/lib.rs``): PyO3 bindings that expose Rust
   functions to Python
-  **Core Logic** (``src/rust_fn/``): Pure Rust implementation of file
   reading and processing

Error Handling
==============

The Rust code uses a comprehensive ``ReadError`` enum that covers all
possible failure modes, from I/O errors to parsing failures. These are
automatically converted to appropriate Python exceptions through the
PyO3 integration.

Dependencies
============

Key Rust dependencies that power the performance:

-  ``ndarray`` - N-dimensional arrays with BLAS integration
-  ``rayon`` - Data parallelism library
-  ``csv`` - Fast CSV parsing
-  ``pyo3`` - Python bindings
-  ``numpy`` - NumPy integration for PyO3
-  ``glob`` - File path pattern matching
-  ``indicatif`` - Progress bars for long operations
