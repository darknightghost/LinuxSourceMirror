#! /usr/bin/env python3
# -*- coding: utf-8 -*-

import pkgutil
import sys
import importlib
import pathli
import threading
import sys
import runstatus


class Client:
    """Base class of all client protocols."""
    def __init__(self, data_path, distros):
        """Constructor.

        :param  data_path:  Path of data directory, :class:`str` object.
        :param  distros:    Distros use this protocol, :class:`list` object.
        """
        super().__init__(self)
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

    def distro_path(self, distro):
        """Get path of distro.

        :param  distro:     Name of distro, :class:`str` object.
        :return:    Path.
        :rtype:     :class:`str`
        """
        path = pathlib.Path(self._data_path) / pathlib.Path(distro)
        if not path.exists():
            path.mkdir(0o775, parents=True, exist_ok=True)

        return str(path.absolute())

    def start(self):
        """Start service.
        """
        raise NotImplementedError()

    def start_work(self):
        """Start service.
        """
        threading.Thread.start()

    def run(self):
        """Run service.
        """
        try:
            self.work()

        except Exception as e:
            logging.exception(sys.exc_info())
            runstatus.exit(-1)

    def work(self):
        """Working thread.
        """
        raise NotImplementedError()

    def stop(self):
        """Stop service.
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
