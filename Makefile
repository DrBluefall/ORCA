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
MAKEFILE_PATH := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))/target

# Where to output build artifacts.
TARGET_PATH ?= $(abspath $(MAKEFILE_PATH))

# Flags to pass to 'cargo build'.
CARGO_BUILD_FLAGS ?=

# Compile in release mode or not.
RELEASE_BUILD ?=

ifeq ($(RELEASE_BUILD),undefined)
	ARTIFACT_PATH := $(realpath $(TARGET_PATH)/debug)
else
	ARTIFACT_PATH := $(realpath $(TARGET_PATH)/release)
	CARGO_BUILD_PATH += --release
endif

###########################
# ORCA Server Build Rules #
###########################

$(ARTIFACT_PATH)/orca-server:
	@echo "]]] Building webserver binary in '${TARGET_PATH}'... [[["
	$(CARGO) build -p orca-server --target-dir '${TARGET_PATH}' $(CARGO_BUILD_FLAGS)

server: $(ARTIFACT_PATH)/orca-server

run-server: $(ARTIFACT_PATH)/orca-server
	$(CARGO) run -p orca-server

clean-server:
	$(CARGO) clean -p orca-server

################################
# ORCA Discord Bot Build Rules #
################################

$(ARTIFACT_PATH)/orca-bot:
	@echo "]]] Building discord bot binary in '${TARGET_PATH}'... [[["
	$(CARGO) build -p orca-bot --target-dir '${TARGET_PATH}' $(CARGO_BUILD_FLAGS)

bot: $(ARTIFACT_PATH)/orca-bot

run-bot: $(ARTIFACT_PATH)/orca-bot
	$(CARGO) run -p orca-bot

clean-bot:
	$(CARGO) clean -p orca-bot

.PHONY: run-server run-bot clean-server clean-bot
