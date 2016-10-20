# -*- coding: utf-8 -*-
# -------------------------------------------------------------------
#
# Copyright (c) 2016 Nathan Fiedler
#
# This file is provided to you under the Apache License,
# Version 2.0 (the "License"); you may not use this file
# except in compliance with the License. You may obtain
# a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied. See the License for the
# specific language governing permissions and limitations
# under the License.
#
# -------------------------------------------------------------------
"""Fabric file for installing requirements on FreeBSD."""

import os

from fabric.api import env, run, sudo, task

env.shell = "/bin/sh -c"
env.hosts = ["default"]
env.use_ssh_config = True
if os.path.exists("user_ssh_config"):
    env.ssh_config_path = "user_ssh_config"
else:
    env.ssh_config_path = "ssh_config"


@task
def all():
    """Install everything needed to build magick-rust."""
    sudo("pkg install -q -y git")
    sudo("pkg install -q -y rust")
    sudo("pkg install -q -y cargo")
    sudo("pkg install -q -y ImageMagick-nox11")
    sudo("pkg install pkgconf")
    sudo("pkg install -q -y clang-devel")
    # set LIBCLANG_PATH so rustc can find libclang.so in its hidden place
    # (using the append operation results in 'Unmatched ".' error)
    run("echo 'setenv LIBCLANG_PATH /usr/local/llvm-devel/lib' >> .cshrc")
