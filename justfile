# Lists available just commands
default:
    @just --list

# Documentation commands - delegates to docs/justfile
docs command *args:
    cd docs && just {{command}} {{args}}

# Build README.rst based off the documentation site
build_readme:
    cat docs/readme_top.rst docs/readme_bottom.rst > README.rst
