import os
import sys
from recommonmark.transform import AutoStructify

# Project information
project = 'Pathmaster'
copyright = '2025, Justin Wayne Liles'
author = 'Justin Wayne Liles'
version = '0.2.7'
release = '0.2.7'

# General configuration
extensions = [
    'recommonmark',
    'sphinx.ext.autodoc',
    'sphinx.ext.viewcode',
    'sphinx.ext.napoleon',
]

templates_path = ['_templates']
exclude_patterns = ['_build', 'Thumbs.db', '.DS_Store']

# HTML output configuration
html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# Recommonmark configuration for Markdown support
def setup(app):
    app.add_config_value('recommonmark_config', {
        'auto_toc_tree_section': 'Contents',
        'enable_eval_rst': True,
    }, True)
    app.add_transform(AutoStructify)

# Source file type configuration
source_suffix = {
    '.rst': 'restructuredtext',
    '.md': 'markdown',
}

master_doc = 'index'