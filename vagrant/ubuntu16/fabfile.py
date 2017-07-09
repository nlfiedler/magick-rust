# -*- coding: utf-8 -*-
# -------------------------------------------------------------------
#
# Copyright (c) 2016-2017 Nathan Fiedler
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
"""Fabric file for installing requirements on Ubuntu Linux."""

import os

from fabric.api import cd, env, run, sudo, task

env.hosts = ["default"]
env.use_ssh_config = True
if os.path.exists("user_ssh_config"):
    env.ssh_config_path = "user_ssh_config"
else:
    env.ssh_config_path = "ssh_config"


@task
def all():
    """Install everything needed to build magick-rust."""
    sudo('apt-get -q -y install git')
    sudo('apt-get -q -y install pkg-config')
    # need the latest possible release of rust for bindgen to work
    run('wget -O rustup-init https://sh.rustup.rs')
    run('chmod +x rustup-init')
    run('./rustup-init -y')
    run('rm -f rustup-init')
    sudo('apt-get -q -y build-dep imagemagick')
    run('wget -q https://www.imagemagick.org/download/ImageMagick-6.9.8-10.tar.gz')
    run('tar zxf ImageMagick-6.9.8-10.tar.gz')
    with cd('ImageMagick-*'):
        run('./configure')
        run('make')
        sudo('make install')
    run('rm -rf ImageMagick*')
    sudo('apt-get -q -y install clang libclang-dev')
    # set LIBCLANG_PATH so rustc can find libclang.so in its hidden place
    # (using the append operation results in 'Unmatched ".' error)
    run("echo 'export LIBCLANG_PATH=/usr/lib/llvm-3.8/lib' >> .bashrc")
