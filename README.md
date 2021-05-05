pdf-form-replace-font
=====================

Replace a given font in a PDF’s text fields with a different one.


## Usage
The following command replaces Helvetica with Courier New (font names are
specified by their PostScript names):

	$ pdf-form-replace-font \
		--find HelveticaLTStd-Bold \
		--replace CourierNewPSMT \
		--output f1040-courier.pdf \
		f1040.pdf


## Install
On Mac OS X, PDF-Form-Replace-Font can be installed with Homebrew:

	$ brew install teddywing/formulae/pdf-form-replace-font

To compile from source or install on other platforms:

	$ cargo install --git https://github.com/teddywing/pdf-form-replace-font.git


## Uninstall

	$ cargo uninstall pdf-form-replace-font


## License
Copyright © 2021 Teddy Wing. Licensed under the GNU GPLv3+ (see the included
COPYING file).
