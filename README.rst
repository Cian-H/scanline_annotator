#####################
 scanline_annotator
#####################

.. image:: https://github.com/Cian-H/scanline_annotator/workflows/CI/badge.svg
  :target: https://github.com/Cian-H/scanline_annotator/actions/workflows/CI.yml

.. image:: https://github.com/Cian-H/scanline_annotator/workflows/Python/badge.svg
  :target: https://github.com/Cian-H/scanline_annotator/actions/workflows/Python.yml

.. image:: https://github.com/Cian-H/scanline_annotator/workflows/Rust/badge.svg
  :target: https://github.com/Cian-H/scanline_annotator/actions/workflows/Rust.yml

.. image:: https://img.shields.io/pypi/dm/scanline-annotator.svg
  :target: https://pypi.python.org/pypi/scanline-annotator

.. image:: https://img.shields.io/github/tag/Cian-H/scanline_annotator.svg
  :target: https://github.com/Cian-H/scanline_annotator/releases

.. image:: https://img.shields.io/github/license/Cian-H/scanline_annotator.svg
  :target: https://github.com/Cian-H/scanline_annotator/blob/main/LICENSE

.. image:: https://readthedocs.org/projects/scanline-annotator/badge/?version=latest
  :target: https://scanline-annotator.readthedocs.io/en/latest/?badge=latest

.. image:: https://coveralls.io/repos/github/Cian-H/scanline_annotator/badge.svg?branch=main
  :target: https://coveralls.io/github/Cian-H/scanline_annotator?branch=main


.. image:: https://img.shields.io/badge/code%20style-Ruff-D7FF64.svg
  :target: https://github.com/astral-sh/ruff

----

A library for fast reading of layer data from the aconity mini powder
bed fusion machine.

##########
 Overview
##########

``scanline_annotator`` is a high-performance Python library for reading
and processing layer data from Aconity mini powder bed fusion machines.
It's built with Rust for maximum performance and uses PyO3 for seamless
Python integration.

##########
 Features
##########

-  **Fast**: Built with Rust for high-performance data processing
-  **Simple**: Easy-to-use Python API
-  **Parallel**: Leverages Rayon for parallel processing of multiple
   files
-  **Type-safe**: Full type annotations and stub files included

###############
 Quick Example
###############

.. code:: python

   import scanline_annotator as ral
   import numpy as np

   # Read all layers from a directory
   data = ral.read_layers("/path/to/layer/files/")

   # Read specific layer files
   files = ["/path/to/0.01.pcd", "/path/to/0.02.pcd"]
   data = ral.read_selected_layers(files)

   # Read a single layer
   layer = ral.read_layer("/path/to/0.01.pcd")
