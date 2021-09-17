#!/bin/bash

mkdir -p out
latexmk -output-directory=out -pdfdvi document.tex
