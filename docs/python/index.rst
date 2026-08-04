######################
 Python API Reference
######################

This section contains the complete Python API reference for
``scanline_annotator``.

********
 Module
********

.. automodule:: scanline_annotator
   :members:
   :undoc-members:
   :show-inheritance:

*************************
 Return Type Information
*************************

The library includes comprehensive type stubs for full IDE support and
type checking.

All functions return NumPy arrays with the following structure:

-  **Column 0**: X coordinates (corrected)
-  **Column 1**: Y coordinates (corrected)
-  **Column 2**: Z coordinates (layer height)
-  **Column 3**: Pyrometer 1 readings
-  **Column 4**: Pyrometer 2 readings
