# patched python-docx
# https://github.com/python-openxml/python-docx/pull/673/files
from docx import Document
import sys

document = Document(sys.argv[1])

document.sections[0].header.paragraphs[0].text = sys.argv[2]

document.save(sys.stdout.buffer)
