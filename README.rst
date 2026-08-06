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

A high-performance library for processing and annotating Powder Bed Fusion (PBF) raster scanlines in-memory.

##########
 Overview
##########

``scanline_annotator`` is a Python library for fast in-memory annotation of raster scanlines from 2D coordinate paths.
It uses dynamic angular topology and median hatch spacing thresholding to adaptively classify scanlines regardless of part orientation.
It is built with Rust for maximum performance and uses PyO3 for seamless NumPy integration.

##########
 Features
##########

-  **Fast**: Built with Rust for high-performance data processing.
-  **Adaptive**: Uses angular topology analysis to dynamically classify scanlines without hardcoded length constraints.
-  **Parallel**: Leverages Rayon for multi-threaded dimensional collapse logic.
-  **Simple**: Zero-copy NumPy array input/output via PyO3.

###############
 Quick Example
###############

.. code:: python

   import scanline_annotator
   import numpy as np

   # Load your 1D coordinate arrays (e.g., from a parsed trajectory file)
   x = np.array([0.0, 0.1, 0.2, 0.2, 0.1, 0.0], dtype=np.float64)
   y = np.array([0.0, 0.0, 0.0, 0.1, 0.1, 0.1], dtype=np.float64)

   # Annotate scanlines!
   # Returns a 1D int32 array assigning each coordinate to a scanline group.
   # Non-scanline points (e.g. jumps, contours) are marked with ID -1.
   scanline_ids = scanline_annotator.annotate_scanlines(x, y)
