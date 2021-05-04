MAN_PAGE := doc/pdf-form-replace-font.1


.PHONY: doc
doc: $(MAN_PAGE)

$(MAN_PAGE): $(MAN_PAGE).txt
	a2x --no-xmllint --format manpage $<
