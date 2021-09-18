#!/bin/bash

mkdir -p out
latexmk -output-directory=out -pdf document.tex $@
