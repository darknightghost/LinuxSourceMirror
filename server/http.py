#! /usr/bin/env python3
# -*- coding: utf-8 -*-

import server


class Server(server.Server):
    """Rsync protocol."""
    def __init__(self, data_path, distros):
        """Constructor.

        :param  data_path:  Path of data directory, :class:`str` object.
        :param  distros:    Distros use this protocol, :class:`list` object.
        """
        super().__init__(data_path, distros)

    def default_config():
        """Get default config.
        
        :return:    Config.
        :rtype:     :class:`dict`
        """
        return {"address": "0.0.0.0", "port": 80}

    def name(self=None):
        """Get protocol name.
        
        :return:    Name of protocol.
        :rtype:     :class:`str`
        """
        return "http"
