# Configuration file for the Sphinx documentation builder.
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

def get_release() -> str:
    import subprocess
    pkgid = subprocess.run(["cargo", "pkgid"], capture_output=True).stdout.decode().strip()
    release_start = pkgid.rfind("#") + 1
    pkglabel = pkgid[release_start:]
    verlabel = pkglabel[pkglabel.rfind("@") + 1:] if "@" in pkglabel else pkglabel
    return verlabel


project = "scanline_annotator"
copyright = "2026, Cian Hughes"
author = "Cian Hughes"
release = get_release()

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinxcontrib_rust",
    "sphinx.ext.autodoc",
    "sphinx.ext.autosummary",
    "sphinx.ext.napoleon",
    "sphinx.ext.viewcode",
    "sphinx.ext.intersphinx",
    "sphinx_autodoc_typehints",
    "sphinx_copybutton",
    "myst_parser",
]

exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "pydata_sphinx_theme"

# -- Extension configuration -------------------------------------------------

# Napoleon settings
napoleon_google_style = True
napoleon_numpy_style = True
napoleon_include_init_with_doc = False
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = False
napoleon_use_admonition_for_notes = False
napoleon_use_admonition_for_references = False
napoleon_use_ivar = False
napoleon_use_param = True
napoleon_use_rtype = True
napoleon_preprocess_types = False
napoleon_type_aliases = None
napoleon_attr_annotations = True

# Autodoc settings
autodoc_typehints = "description"
autodoc_member_order = "bysource"
autosummary_generate = True

# Intersphinx mapping
intersphinx_mapping = {
    "python": ("https://docs.python.org/3", None),
    "numpy": ("https://numpy.org/doc/stable/", None),
}

# Sphinx-rust settings
rust_crates = {
    "scanline_annotator": ".",
}
rust_doc_dir = "rust/crates"
rust_rustdoc_fmt = "md"

# MyST settings
myst_enable_extensions = [
    "deflist",
    "html_image",
    "attrs_block",
    "colon_fence",
    "html_admonition",
    "replacements",
    "smartquotes",
    "strikethrough",
    "tasklist",
]

# Copy button settings
copybutton_prompt_text = r">>> |\.\.\. |\$ |In \[\d*\]: | {2,5}\.\.\.: | {5,8}: "
copybutton_prompt_is_regexp = True
