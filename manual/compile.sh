#!/bin/bash

function borkify {
	convert -density 200 "$1" -rotate "$(python -c 'import random; print("{:f}".format((random.random() - 0.5)*0.2))')" +noise Multiplicative -format pdf -quality 85 -compress JPEG -colorspace gray "$2"
}

set -e # abort on any error
mkdir -p out
latexmk -output-directory=out -pdf document.tex $@
borkify "out/document.pdf" "out/document-scan.pdf"
borkify "out/document-scan.pdf" "out/document-scan.pdf"
borkify "out/document-scan.pdf" "out/document-scan.pdf"
