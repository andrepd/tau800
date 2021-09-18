#!/bin/bash

set -e # abort on any error
mkdir -p out
latexmk -output-directory=out -pdf document.tex $@
convert -density 200 out/document.pdf -rotate 0.1 +noise Multiplicative -format pdf -quality 85 -compress JPEG -colorspace gray out/document-scan.pdf
