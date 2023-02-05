# Copyright (C) Alexander Bisono.
#
# This file is a part of O.R.C.A., the Omniscient Recording Computer of Alterna.
#
# O.R.C.A is free software: you can redistribute it and/or modify it under the
# terms of the GNU Affero General Public License as published by the Free
# Software Foundation, either version 3 of the License, or (at your option) any
# later version.
#
# O.R.C.A is distributed in the hope that it will be useful, but WITHOUT ANY
# WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
# A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
# details.
#
# You should have received a copy of the GNU Affero General Public License
# along with O.R.C.A. If not, see <https://www.gnu.org/licenses/>.

# The cargo executable to use.
CARGO ?= cargo

# The path to this Makefile.
MAKEFILE_PATH := $(abspath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

# Where to output build artifacts.
TARGET_PATH ?= $(MAKEFILE_PATH)/target

# Flags to pass to 'cargo build'.
CARGO_BUILD_FLAGS ?=

# Where should Vite place its output files?
VITE_OUTPUT_PATH ?= $(TARGET_PATH)/frontend-dist
export VITE_OUTPUT_PATH

# Compile in release mode or not.
RELEASE_BUILD ?=

ifeq ($(RELEASE_BUILD),undefined)
	ARTIFACT_PATH := $(TARGET_PATH)/debug
else
	ARTIFACT_PATH := $(TARGET_PATH)/release
	CARGO_BUILD_FLAGS += --release
endif

############################
# Web Frontend Build Rules #
############################

$(TARGET_PATH)/frontend-dist/index.html: ${shell find webserver/frontend     \
												-not -path '*/node_modules/*'\
												\( -type f -iname '*.ts'     \
													-o -iname '*.js'         \
													-o -iname '*.html'       \
													-o -iname '*.json'       \
													-o -iname '*.svelte'     \)}
	@echo "]]] Writing frontend assets to '${VITE_OUTPUT_PATH}'... [[["
	mkdir -p ${VITE_OUTPUT_PATH}
	cd "${MAKEFILE_PATH}/webserver/frontend" && npm install
	cd "${MAKEFILE_PATH}/webserver/frontend" && npx vite build

frontend: $(TARGET_PATH)/frontend-dist/index.html

clean-frontend:
	rm -rf "${VITE_OUTPUT_PATH}"

#######################
#  Server Build Rules #
#######################

$(ARTIFACT_PATH)/orca-server: ${shell find webserver -iname '*.rs' \
										-o -iname '*.toml' -type f }
	@echo "]]] Building webserver binary in '${TARGET_PATH}'... [[["
	$(CARGO) build -p orca-server --target-dir '${TARGET_PATH}' \
		$(CARGO_BUILD_FLAGS)

server: $(ARTIFACT_PATH)/orca-server

run-server: server frontend orcaconf.toml
	$(CARGO) run -p orca-server

clean-server:
	$(CARGO) clean -p orca-server

###########################
# Discord Bot Build Rules #
###########################

$(ARTIFACT_PATH)/orca-bot: ${shell find bot -iname '*.rs'      \
									-o -iname '*.toml' -type f }
	@echo "]]] Building discord bot binary in '${TARGET_PATH}'... [[["
	$(CARGO) build -p orca-bot --target-dir '${TARGET_PATH}' \
		$(CARGO_BUILD_FLAGS)

bot: $(ARTIFACT_PATH)/orca-bot

run-bot: bot orcaconf.toml
	$(CARGO) run -p orca-bot

clean-bot:
	$(CARGO) clean -p orca-bot

.PHONY: run-server run-bot clean-server clean-bot
