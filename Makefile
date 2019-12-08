CP := cp
INSTALL := install
INSTALL_DATA := $(INSTALL) -m 644
MKDIR = mkdir -p
NPM := npm

prefix := /usr/local

export NODE_ENV

all:
	$(NPM) ci

install:
	$(MKDIR) "$(DESTDIR)$(prefix)/lib/itsbinotstraight"
	$(INSTALL_DATA) -t "$(DESTDIR)$(prefix)/lib/itsbinotstraight/" index.js tweet.js biwords .env
	$(CP) -r node_modules/ "$(DESTDIR)$(prefix)/lib/itsbinotstraight/"
	$(MKDIR) "$(DESTDIR)$(prefix)/lib/systemd/system/"
	$(INSTALL_DATA) -t "$(DESTDIR)$(prefix)/lib/systemd/system/" itsbinotstraight.service itsbinotstraight.timer
