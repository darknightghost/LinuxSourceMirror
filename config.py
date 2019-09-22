#! /usr/bin/env python3
# -*- coding: utf-8 -*-

import json
import client
import server
import logging
import copy


class __Config:
    '''
        Config class.
    '''
    def __init__(self):
        """Initialized an empty config."""
        # Make default options
        self.__base_configs = {
            "log_path": "./test/log",  # Path of log file.
            "log_level": logging.INFO,  # Log level.
            "max_log_file_num": 10,  # Maxium number of log file.
            "max_log_file_size": 1024,  # Maxium size of log file.
            "data_path": "/mirror",  # Path of data.
            "uid": 1000,  # UID, only available when running as daemon.
            "gid": 1000  # GID, only available when running as daemon.
        }

        # Client protocols
        self.__client_protocols = {}  # Client protocols.
        for m in client.get_protocols():
            self.__client_protocols[
                m.Client.name()] = m.Client.default_config()

        self.__server_protocols = {}  # Server protocols.
        for m in server.get_protocols():
            self.__server_protocols[
                m.Server.name()] = m.Server.default_config()

        self.__distros = {}  # Distros.

    def load_config(self, path):
        """ Load config file.

            :param  path:   Path of config file, :class:`str`.
        """
        with open(path, encoding="utf-8") as f:
            jsonData = json.loads(f.read(), encoding="utf-8")

        # Load base configs
        for k in jsonData:
            if k in self.__base_configs:
                newVal = jsonData[k]
                if type(newVal) != type(self.__base_configs[k]):
                    raise TypeError("Option %s in config file \"%s\" should " \
                            "be an instance of \"%s\"."%(k, path,
                                str(type(self.__base_configs[k]))))

            elif k not in ("client_protocols", "server_protocols", "distros"):
                logging.warning("Unknow option \"%s\" in config file \"%s\"!" %
                                (k, path))

        # Load clent configs
        if "client_protocols" in jsonData:
            for k in jsonData["client_protocols"]:
                if k in self.__client_protocols:
                    protocol_cfg = self.__client_protocols[k]
                    for c in jsonData["client_protocols"][k]:
                        if c in protocol_cfg:
                            protocol_cfg[c] = jsonData["client_protocols"][k][
                                c]

                        else:
                            logging.warning(
                                "Unknow option \"%s\" in client protocol \"%s\" in config file \"%s\"!"
                                % (c, k, path))

                else:
                    logging.warning(
                        "Unknow client protocl \"%s\" in config file \"%s\"!" %
                        (k, path))

        # Load server configs
        if "server_protocols" in jsonData:
            for k in jsonData["server_protocols"]:
                if k in self.__server_protocols:
                    protocol_cfg = self.__server_protocols[k]
                    for s in jsonData["server_protocols"][k]:
                        if s in protocol_cfg:
                            protocol_cfg[s] = jsonData["server_protocols"][k][
                                s]

                        else:
                            logging.warning(
                                "Unknow option \"%s\" in server protocol \"%s\" in config file \"%s\"!"
                                % (s, k, path))

                else:
                    logging.warning(
                        "Unknow server protocl \"%s\" in config file \"%s\"!" %
                        (k, path))

        # Load distro configs
        try:
            for k in jsonData["distros"]:
                pass

        except KeyError:
            raise KeyError("Key \"distros\" is required in  conmfig file!")

    def log_path(self):
        """Get path of log file.

        :return:    Path of log file.
        :rtype:     :class: `str`
        """
        return self.__base_configs["log_path"]

    def log_level(self):
        """Get log level.

        :return:    Log level.
        :rtype:     :class: `int`
        """
        return self.__base_configs["log_level"]

    def max_log_file_num(self):
        """Get maxium count of log file.

        :return:    Maxium count of log file.
        :rtype:     :class: `int`
        """
        return self.__base_configs["max_log_file_num"]

    def max_log_file_size(self):
        """Get maxium size of each log file.

        :return:    Maxium size of each log file.
        :rtype:     :class: `int`
        """
        return self.__base_configs["max_log_file_size"]

    def data_path(self):
        """Get path of data.

        :return:    Path of data.
        :rtype:     :class: `str`
        """
        return self.__base_configs["data_path"]

    def uid(self):
        """Get UID.

        :return:    UID.
        :rtype:     :class: `str`
        """
        return self.__base_configs["uid"]

    def gid(self):
        """Get GID.

        :return:    GID.
        :rtype:     :class: `str`
        """
        return self.__base_configs["gid"]

    def client_protocol_cfg(self, name):
        """Get config of client protocol.

        :param      name:   Name of protocol, :class:`str`.
        :return:    Config of server protocol.
        :rtype:     :class: `dict`
        """
        return copy.copy(self.__client_protocols[name])

    def server_protocol_cfg(self, name):
        """Get config of server protocol.

        :param      name:   Name of protocol, :class:`str`.
        :return:    Config of server protocol.
        :rtype:     :class: `dict`
        """
        return copy.copy(self.__server_protocols[name])

    def distros(self):
        """Get list of distro names.

        :return:    Names of distros.
        :rtype:     :class: `list`
        """
        ret = []
        for d in self.__distros:
            ret.append(d)

        return ret

    def distro_cfg(self, name):
        """Get config of server protocol.

        :param      name:   Name of distro, :class:`str`.
        :return:    Config of server protocol.
        :rtype:     :class: `str`
        """
        return self.__distros[name]


config = __Config()
