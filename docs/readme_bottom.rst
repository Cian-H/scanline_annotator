##########
 Overview
##########

``scanline_annotator`` is a Python library for fast in-memory annotation
of raster scanlines from 2D coordinate paths. It uses dynamic angular
topology and median hatch spacing thresholding to adaptively classify
scanlines regardless of part orientation. It is built with Rust for
maximum performance and uses PyO3 for seamless NumPy integration.

##########
 Features
##########

-  **Fast**: Built with Rust for high-performance data processing.
-  **Adaptive**: Uses angular topology analysis to dynamically classify
   scanlines without hardcoded length constraints.
-  **Parallel**: Leverages Rayon for multi-threaded dimensional collapse
   logic.
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
