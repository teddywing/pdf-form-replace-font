# Copyright (c) 2021  Teddy Wing
#
# This file is part of PDF Form Replace Font.
#
# PDF Form Replace Font is free software: you can redistribute it
# and/or modify it under the terms of the GNU General Public License
# as published by the Free Software Foundation, either version 3 of
# the License, or (at your option) any later version.
#
# PDF Form Replace Font is distributed in the hope that it will be
# useful, but WITHOUT ANY WARRANTY; without even the implied warranty
# of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
# General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with PDF Form Replace Font. If not, see
# <https://www.gnu.org/licenses/>.


VERSION := $(shell egrep '^version = ' Cargo.toml | awk -F '"' '{ print $$2 }')
TOOLCHAIN := $(shell fgrep default_host_triple $(HOME)/.rustup/settings.toml | awk -F '"' '{ print $$2 }')

SOURCES := $(shell find src -name '*.rs')
RELEASE_PRODUCT := target/release/pdf-form-replace-font

MAN_PAGE := doc/pdf-form-replace-font.1

DIST := $(abspath dist)
DIST_PRODUCT := $(DIST)/bin/pdf-form-replace-font
DIST_MAN_PAGE := $(DIST)/share/man/man1/pdf-form-replace-font.1


$(RELEASE_PRODUCT): $(SOURCES)
	cargo build --release


.PHONY: doc
doc: $(MAN_PAGE)

$(MAN_PAGE): $(MAN_PAGE).txt
	a2x --no-xmllint --format manpage $<


.PHONY: dist
dist: $(DIST_PRODUCT) $(DIST_MAN_PAGE)

$(DIST):
	mkdir -p $@

$(DIST)/bin: | $(DIST)
	mkdir -p $@

$(DIST)/share/man/man1: | $(DIST)
	mkdir -p $@

$(DIST_PRODUCT): $(RELEASE_PRODUCT) | $(DIST)/bin
	cp $< $@

$(DIST_MAN_PAGE): $(MAN_PAGE) | $(DIST)/share/man/man1
	cp $< $@


.PHONY: pkg
pkg: pdf-form-replace-font_$(VERSION)_$(TOOLCHAIN).tar.bz2

pdf-form-replace-font_$(VERSION)_$(TOOLCHAIN).tar.bz2: dist
	tar cjv -s /dist/pdf-form-replace-font_$(VERSION)_$(TOOLCHAIN)/ -f $@ dist
