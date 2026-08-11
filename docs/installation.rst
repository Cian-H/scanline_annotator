##############
 Installation
##############

**************
 Requirements
**************

-  Python 3.11 or higher
-  NumPy 2.0.0 or higher

*************************
 From PyPI (Recommended)
*************************

.. code:: bash

   pip install scanline-annotator

*************
 From Source
*************

If you want to build from source or contribute to the project:

Prerequisites
=============

-  Rust 1.70 or higher
-  Python 3.11 or higher
-  uv (for development)

Build Steps
===========

#. Clone the repository:

   .. code:: bash

      git clone https://github.com/Cian-H/scanline_annotator.git
      cd scanline_annotator

#. Install Python dependencies:

   .. code:: bash

      uv sync

#. Build the Rust extension:

   .. code:: bash

      uv run maturin develop

#. Run tests to verify installation:

   .. code:: bash

      uv run pytest

**************************
 Development Installation
**************************

For development work, you'll also want the development dependencies:

.. code:: bash

   uv sync --group dev --group docs

This installs additional tools for:

-  Code formatting (ruff)
-  Type checking (ty)
-  Testing (pytest)
-  Documentation building (sphinx)

*****************
 Troubleshooting
*****************

Common Issues
=============

**Import Error**: If you get import errors, make sure you've run
``maturin develop`` to build the Rust extension.

**Performance Issues**: The library uses parallel processing by default.
If you encounter memory issues with very large datasets, consider
processing your data arrays in chunks.

**Rust Compilation Errors**: Make sure you have a recent version of Rust
installed. The minimum supported version is 1.70.

Platform Notes
==============

**Windows**: You may need to install the Microsoft C++ Build Tools if
you don't already have them.

**macOS**: Xcode command line tools are required for Rust compilation.

**Linux**: Most distributions should work out of the box. You may need
to install ``build-essential`` on Debian/Ubuntu systems.
