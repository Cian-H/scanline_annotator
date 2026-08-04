##################
 Quickstart Guide
##################

This guide will get you up and running with ``scanline_annotator`` in
just a few minutes.

*************
 Basic Usage
*************

Reading All Layers from a Directory
===================================

The most common use case is reading all layer files from a directory:

.. code:: python

   import scanline_annotator as ral
   import numpy as np

   # Read all .pcd files from a directory
   data = ral.read_layers("/path/to/your/layer/files/")

   print(f"Loaded {data.shape[0]} data points")
   print(f"Data shape: {data.shape}")
   print(f"Columns: [x, y, z, data1, data2]")

Reading Specific Files
======================

If you want to read only specific layer files:

.. code:: python

   import scanline_annotator as ral

   # List of specific files to read
   files = ["/path/to/0.1.pcd", "/path/to/0.2.pcd", "/path/to/0.3.pcd"]

   data = ral.read_selected_layers(files)

Reading a Single Layer
======================

For processing individual layers:

.. code:: python

   import scanline_annotator as ral

   # Read just one layer file
   layer_data = ral.read_layer("/path/to/0.01.pcd")

   # Extract coordinates
   x_coords = layer_data[:, 0]
   y_coords = layer_data[:, 1]
   z_coords = layer_data[:, 2]

***********************
 Working with the Data
***********************

Understanding the Data Format
=============================

All functions return NumPy arrays with 5 columns:

-  **Column 0**: X coordinates (corrected)
-  **Column 1**: Y coordinates (corrected)
-  **Column 2**: Z coordinates (layer height)
-  **Column 3**: Original data column 3
-  **Column 4**: Original data column 4

The X and Y coordinates are automatically corrected using the
calibration formulas built into the library.

Example: Basic Data Analysis
============================

.. code:: python

   import scanline_annotator as ral
   import numpy as np
   import matplotlib.pyplot as plt

   # Load data
   data = ral.read_layers("/path/to/layers/")

   # Basic statistics
   print(f"X range: {data[:, 0].min():.2f} to {data[:, 0].max():.2f}")
   print(f"Y range: {data[:, 1].min():.2f} to {data[:, 1].max():.2f}")
   print(f"Z range: {data[:, 2].min():.2f} to {data[:, 2].max():.2f}")

   # Plot layer distribution
   unique_z = np.unique(data[:, 2])
   layer_counts = [np.sum(data[:, 2] == z) for z in unique_z]

   plt.figure(figsize=(10, 6))
   plt.plot(unique_z, layer_counts)
   plt.xlabel("Layer Height (Z)")
   plt.ylabel("Number of Points")
   plt.title("Points per Layer")
   plt.show()

Example: Processing by Layer
============================

.. code:: python

   import scanline_annotator as ral
   import numpy as np

   # Read data
   data = ral.read_layers("/path/to/layers/")

   # Group by Z coordinate (layer)
   unique_z = np.unique(data[:, 2])

   layer_stats = []
   for z in unique_z:
       layer_mask = data[:, 2] == z
       layer_points = data[layer_mask]

       stats = {
           "z": z,
           "point_count": len(layer_points),
           "x_mean": layer_points[:, 0].mean(),
           "y_mean": layer_points[:, 1].mean(),
           "data1_mean": layer_points[:, 3].mean(),
           "data2_mean": layer_points[:, 4].mean(),
       }
       layer_stats.append(stats)

   # Convert to structured array for easier analysis
   layer_stats = np.array(layer_stats)

******************
 Performance Tips
******************

Parallel Processing
===================

The library automatically uses parallel processing for multiple files.
For best performance:

-  Use ``read_layers()`` for directories with many files
-  The library will automatically use all available CPU cores
-  Larger numbers of files will see better speedup

Memory Usage
============

For very large datasets:

-  Consider processing files in batches if memory is limited
-  Use ``read_selected_layers()`` to process subsets
-  The library streams data efficiently, but the final arrays are held
   in memory

File Organization
=================

For optimal performance:

-  Keep layer files in a single directory when using ``read_layers()``
-  Use consistent naming (the Z coordinate is extracted from the
   filename)
-  Ensure files are properly formatted space-delimited text

****************
 Error Handling
****************

The library provides detailed error messages for common issues:

.. code:: python

   import scanline_annotator as ral

   try:
       data = ral.read_layers("/path/to/layers/")
   except IOError as e:
       print(f"File read error: {e}")
   except RuntimeError as e:
       print(f"Processing error: {e}")

************
 Next Steps
************

-  Check out the full :doc:`python/index` for detailed function
   documentation
-  See :doc:`development` if you want to contribute to the project
-  For performance-critical applications, review the :doc:`rust/index`
