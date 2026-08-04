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
