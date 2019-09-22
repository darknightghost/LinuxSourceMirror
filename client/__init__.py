#! /usr/bin/env python3
# -*- coding: utf-8 -*-

import pkgutil
import sys
import importlib


class Client:
    """Base class of all client protocols."""
    def __init__(self, data_path, distros):
        """Constructor.

        :param  data_path:  Path of data directory, :class:`str` object.
        :param  distros:    Distros use this protocol, :class:`list` object.
        """
        self._data_path = data_path
        self._distros = distros

    def default_config():
        """Get default config.
        
        :return:    Config.
        :rtype:     :class:`dict`
        """
        raise NotImplementedError()

    def name(self=None):
        """Get protocol name.
        
        :return:    Name of protocol.
        :rtype:     :class:`str`
        """
        raise NotImplementedError()


def get_protocols():
    """List all protocols.

    :return:    Name of all protocol.
    :rtype:     :class:`list`
    """
    ret = []
    for filefiner, name, ispkg \
            in pkgutil.iter_modules(__path__, "%s."%(__name__)):
        m = importlib.import_module(name, package=sys.modules[__name__])
        if "Client" in dir(m):
            ret.append(m)

    return ret
