#!/usr/bin/env python
# patched python-docx
# https://github.com/python-openxml/python-docx/pull/673/files
from docx import Document
from docx.shared import RGBColor, Pt
import sys

document = Document(sys.argv[1])

paragraph = document.sections[0].header.paragraphs[0]
paragraph.text = sys.argv[2]
paragraph.style.font.color.rgb = RGBColor(0xff, 0xff, 0xff)
paragraph.style.font.size = Pt(1)

document.save(sys.stdout.buffer)
