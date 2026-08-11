##################
 Quickstart Guide
##################

This guide will get you up and running with ``scanline_annotator`` in
just a few minutes.

*************
 Basic Usage
*************

In-Memory Scanline Annotation
=============================

The most common use case is feeding 1D coordinate arrays to the library
for scanline classification. The arrays can be created directly from
parsed trajectory files or any standard Python processing step.

.. code:: python

   import scanline_annotator
   import numpy as np

   # 1D coordinate arrays (float64 or float32)
   x = np.array([0.0, 0.1, 0.2, 0.2, 0.1, 0.0], dtype=np.float64)
   y = np.array([0.0, 0.0, 0.0, 0.1, 0.1, 0.1], dtype=np.float64)

   # The library automatically groups points belonging to parallel raster lines
   scanline_ids = scanline_annotator.annotate_scanlines(x, y)

   print(f"Assigned IDs: {scanline_ids}")

***********************
 Working with the Data
***********************

Understanding the Output
========================

The function ``annotate_scanlines(x, y)`` returns a NumPy array with the
same length as the input arrays. The returned array has type ``int32``.

-  **Scanline IDs**: Continuous blocks of raster tracks are assigned a
   positive integer ID (e.g., 1, 2, 3).

-  **Non-scanline Points**: Transition tracks, contour passes, or jump
   vectors that do not match rastering topography are marked with a
   value of ``-1``.

Example: Basic Data Analysis
============================

.. code:: python

   import scanline_annotator
   import numpy as np
   import matplotlib.pyplot as plt

   # Load your large real-world dataset
   # For example, np.load('trajectory.npz')
   x = np.random.rand(100_000)
   y = np.random.rand(100_000)

   # Annotate
   labels = scanline_annotator.annotate_scanlines(x, y)

   # Filter out non-scanline points
   raster_mask = labels != -1

   print(f"Total points: {len(x)}")
   print(f"Raster points: {np.sum(raster_mask)}")

   # Plot the first isolated scanline
   if np.any(labels == 1):
       scanline_1_x = x[labels == 1]
       scanline_1_y = y[labels == 1]

       plt.figure(figsize=(10, 6))
       plt.plot(scanline_1_x, scanline_1_y, marker='o')
       plt.title("Scanline #1")
       plt.show()

******************
 Performance Tips
******************

Parallel Processing
===================

The library automatically utilizes Rayon for parallel processing when
collapsing array dimensions. For very large arrays (e.g. over 500,000
points), the workload is distributed across all available CPU cores.

Memory Usage
============

The algorithm operates efficiently without deep copies where possible,
but generating the output array and some internal spatial data
structures requires RAM proportional to the size of your input arrays. A
typical 3.5 million point dataset will take about 15-20 MB of auxiliary
memory during processing.

****************
 Error Handling
****************

The library provides structured error bubbling from Rust directly into
Python exceptions:

.. code:: python

   import scanline_annotator
   import numpy as np

   try:
       # Providing unequal length arrays
       x = np.array([0.0, 1.0])
       y = np.array([0.0])
       labels = scanline_annotator.annotate_scanlines(x, y)
   except ValueError as e:
       # "Miscellaneous Error: Input x and y length mismatch: 2 vs 1"
       print(f"Processing error: {e}")
   except TypeError as e:
       # e.g., passing int arrays instead of float
       print(f"Type error: {e}")

************
 Next Steps
************

-  Check out the full :doc:`python/index` for detailed function
   documentation
-  See :doc:`development` if you want to contribute to the project
-  For performance-critical applications, review the :doc:`rust/index`
