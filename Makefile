NAME = 01

BINDIR = bin

SRCDIRS = $(shell find . -maxdepth 1 -name '[0-9][0-9]')
MAIN = main.rs
MAINS = $(addsuffix /$(MAIN),$(SRCDIRS))
OPTLEVEL = 0

BINS = $(SRCDIRS:%=$(BINDIR)/%)

all: $(BINDIR) $(BINS)

$(BINDIR)/%: %
	rustc -C opt-level=$(OPTLEVEL) -o $@ $</$(MAIN)

$(BINDIR):
	mkdir $@

fmt: $(MAINS)
	rustfmt $^

clean:
	rm $(BINS)
	rmdir $(BINDIR)

re: clean all
