#! /usr/bin/env python3
# -*- coding: utf-8 -*-

import client
import config
import urllib
import urllib.parse
import logging
import time
import subprocess
import signal


class Client(client.Client):
    """Rsync protocol."""
    SLEEP_TIME = 0.5
    EXIT_CODES = {
        1: "Syntax or usage error",
        2: "Protocol incompatibility",
        3: "Errors selecting input/output files, dirs",
        4:
        "Requested action not supported: an attempt was made to manipulate 64-bit files on a platform that cannot support them; or an option was specified that is supported by the client and not by the server.",
        5: "Error starting client-server protocol",
        6: "Daemon unable to append to log-file",
        10: "Error in socket I/O",
        11: "Error in file I/O",
        12: "Error in rsync protocol data stream",
        13: "Errors with program diagnostics",
        14: "Error in IPC code",
        20: "Received SIGUSR1 or SIGINT",
        21: "Some error returned by waitpid()",
        22: "Error allocating core memory buffers",
        23: "Partial transfer due to error",
        24: "Partial transfer due to vanished source files",
        25: "The --max-delete limit stopped deletions",
        30: "Timeout in data send/receive"
    }

    def __init__(self, *args):
        """Constructor.

        :param  data_path:  Path of data directory, :class:`str` object.
        :param  distros:    Distros use this protocol, :class:`list` object.
        """
        super().__init__(*args)

    def default_config():
        """Get default config.
        
        :return:    Config.
        :rtype:     :class:`dict`
        """
        return {
            "exec": "rsync",
            "interval": 3600,
            "max_connection": 10,
            "connnect-timeout": 30,
            "timeout": 30
        }

    def name(self=None):
        """Get protocol name.
        
        :return:    Name of protocol.
        :rtype:     :class:`str`
        """
        return "rsync"

    def start(self):
        """Start service.
        """
        self.__config = config.config.client_protocol_cfg(self.name())
        self.__run = True
        self.start_work()

    def stop(self):
        """Stop service.
        """
        self.__run = False
        self.join()

    def work(self):
        """Working thread
        """
        self.__tasks = {}
        self.__seconds = {}
        selt.__contimeout = self.__config["connnect-timeout"]
        selt.__timeout = self.__config["timeout"]

        for distro in self._distros:
            self.__seconds[distro] = self.__config["interval"]

        while self.__run:
            time.sleep(self.SLEEP_TIME)
            self.__poll()
            for distro in self.__seconds:
                self.__seconds[distro] += self.SLEEP_TIME
                if self.__seconds[distro] >= self.__config["interval"] and len(
                        self.__tasks) < self.__config["max_connection"]:
                    # Run task
                    self.__seconds[distro] = 0.0

                    if distro not in self.__tasks:
                        self.__tasks[distro] = self.mk_task(distro)

        # Kill all tasks
        for distro in list(self.__tasks.keys()):
            self.__tasks[distro].send_signal(signal.SIGINT)
            self.__tasks[distro].wait()
            del self.__tasks[distro]
            logging.info(
                "Synchronization process of distro \"%s\" has been killed." %
                (distro))

    def __poll(self):
        """Poll all tasks.
        """
        for distro in list(self.__tasks.keys()):
            task = self.__tasks[distro]
            exit_code = task.poll()
            if exit_code == None:
                continue

            elif exit_code == 0:
                logging.info("Distro \"%s\" synchronized." % (distro))
                self.__seconds[distro] = 0.0
                del self.__tasks[distro]

            else:
                if exit_code == 20 and not self.__run:
                    break

                try:
                    logging.error("Rsync failed with exit code %d when " \
                            "synchronizing distro \"%s\" from \"%s\" : %s" % (exit_code,
                                distro,
                                config.config.distro_url(distro),
                                self.EXIT_CODES[exit_code]))

                except KeyError:
                    logging.error("Rsync failed with exit code %d when " \
                            "synchronizing distro \"%s\" from \"%s\"" % (exit_code,
                                distro,
                                config.config.distro_url(distro)))

                self.__tasks[distro] = self.mk_task(distro)

    def mk_task(self, name):
        """Get new task.

        :param  name:   Name, :class:`str` object.
        :return:    New task.
        :rtype:     :class:`subprocess.Popen`
        """
        if name not in self._distros:
            raise RuntimeError("Distro \"%s\" not found." % (name))

        path = self.distro_path(name)
        url = config.config.distro_url(name)
        command = self.mk_command(url, path)

        ret = subprocess.Popen(command,
                               stdin=subprocess.DEVNULL,
                               stdout=subprocess.DEVNULL,
                               stderr=subprocess.DEVNULL)
        logging.info("Synchronization of distro \"%s\" started." % (name))

        return ret

    def mk_command(self, url, path):
        """Make rsync command.

        :param  url:    URL, :class:`str` object.
        :param  path    Destination, :class:`str` object.
        :return:    Command.
        :rtype:     :class:`str`
        """
        return [
            self.__config["exec"], "-rtlvH", "--delete-after",
            "--contimeout=%d" % (self.__contimeout),
            "--timeout=%d" % (self.__timeout), "--partial", "--delay-updates",
            "--safe-links", url, path
        ]
