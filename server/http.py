#! /usr/bin/env python3
# -*- coding: utf-8 -*-

import server
import config
import http
import http.server
import socketserver
import copy
import urllib
import html
import sys
import os
import io
import logging
import threading


class RequestHandlerTemplate(http.server.SimpleHTTPRequestHandler):
    DATA_DIRECTORY = None
    DISTROS = None

    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=self.DATA_DIRECTORY, **kwargs)

    def do_GET(self):
        # Environment
        self._distros = self.DISTROS
        self._distros.sort(key=lambda a: a.lower())
        self._data_directory = self.DATA_DIRECTORY
        path = self.path.strip().strip('/').split('/')

        f = None
        if len(path) == 1:
            if path[0] == "":
                f = self.list_root()

            elif path[0] == "favicon.ico":
                self.send_response(200)
                self.end_headers()
                return

            elif path[0] in self._distros:
                f = self.list_directory(
                    os.path.join(self.DATA_DIRECTORY, path[0]))

        else:
            path = os.path.join(self._data_directory, *path)
            if os.path.exists(path):
                if os.path.isdir(path):
                    f = self.list_directory(path)

                else:
                    ctype = self.guess_type(path)
                    try:
                        f = open(path, 'rb')
                        try:
                            fs = os.fstat(f.fileno())
                            pos = 0
                            size = fs[6]
                            print(self.requestline)
                            self.send_response(200)
                            self.send_header("Content-type", ctype)
                            self.send_header("Content-Range",
                                             "%d-%d" % (pos, pos + size - 1))
                            self.send_header("Content-Length", str(size))
                            self.send_header(
                                "Last-Modified",
                                self.date_time_string(fs.st_mtime))
                            self.end_headers()

                        except:
                            f.close()
                            f = None

                    except OSError:
                        f = None

        if f == None:
            self.send_error(404, "File not found")

        else:
            self.copyfile(f, self.wfile)
            f.close()

    def list_root(self):
        """Helper to produce a root directory listing (absent index.html).

        Return value is either a file object, or None (indicating an
        error).  In either case, the headers are sent, making the
        interface the same as for send_head().

        """
        r = []
        path = self._data_directory
        try:
            displaypath = urllib.parse.unquote(self.path,
                                               errors='surrogatepass')
        except UnicodeDecodeError:
            displaypath = urllib.parse.unquote(path)
        displaypath = html.escape(displaypath, quote=False)
        enc = sys.getfilesystemencoding()
        title = 'Directory listing for %s' % displaypath
        r.append('<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" '
                 '"http://www.w3.org/TR/html4/strict.dtd">')
        r.append('<html>\n<head>')
        r.append('<meta http-equiv="Content-Type" '
                 'content="text/html; charset=%s">' % enc)
        r.append('<title>%s</title>\n</head>' % title)
        r.append('<body>\n<h1>%s</h1>' % title)
        r.append('<hr>\n<table border=\"0\" width="100%" align="left">\n')
        r.append(
            '<tr><th align="left">Distro</th><th align="left">Source</th></tr>\n'
        )
        for name in self._distros:
            fullname = os.path.join(path, name)
            displayname = linkname = name
            # Append / for directories or @ for symbolic links
            if os.path.isdir(fullname):
                displayname = name + "/"
                linkname = name + "/"

            r.append(
                '<tr><td><a href="%s">%s</a></td><td><a href="%s">%s</a></td></tr>\n'
                % (urllib.parse.quote(linkname, errors='surrogatepass'),
                   html.escape(displayname,
                               quote=False), config.config.distro_url(name),
                   config.config.distro_url(name)))

        r.append('</table>\n<hr>\n</body>\n</html>\n')
        encoded = '\n'.join(r).encode(enc, 'surrogateescape')
        f = io.BytesIO()
        f.write(encoded)
        f.seek(0)
        self.send_response(200)
        self.send_header("Content-type", "text/html; charset=%s" % enc)
        self.send_header("Content-Length", str(len(encoded)))
        self.end_headers()
        return f

    def list_directory(self, path):
        """Helper to produce a directory listing (absent index.html).

        Return value is either a file object, or None (indicating an
        error).  In either case, the headers are sent, making the
        interface the same as for send_head().

        """
        try:
            list = os.listdir(path)
        except OSError:
            self.send_error(404, "No permission to list directory")
            return None
        list.sort(key=lambda a: a.lower())
        r = []
        try:
            displaypath = urllib.parse.unquote(self.path,
                                               errors='surrogatepass')
        except UnicodeDecodeError:
            displaypath = urllib.parse.unquote(path)
        displaypath = html.escape(displaypath, quote=False)
        enc = sys.getfilesystemencoding()
        title = 'Directory listing for %s' % displaypath
        r.append('<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN" '
                 '"http://www.w3.org/TR/html4/strict.dtd">')
        r.append('<html>\n<head>')
        r.append('<meta http-equiv="Content-Type" '
                 'content="text/html; charset=%s">' % enc)
        r.append('<title>%s</title>\n</head>' % title)
        r.append('<body>\n<h1>%s</h1>' % title)
        r.append('<hr>\n<table border=\"0\" width="100%" align="left">\n')
        r.append(
            '<tr><th align="left">Name</th><th align="left">Size</th><th align="left">Last Modified</th></tr>\n'
        )
        for name in list:
            fullname = os.path.join(path, name)
            displayname = linkname = name
            # Append / for directories or @ for symbolic links
            if os.path.isdir(fullname):
                displayname = name + "/"
                linkname = name + "/"
            if os.path.islink(fullname):
                displayname = name + "@"
                # Note: a link to a directory displays with @ and links with /

            try:
                fs = os.stat(fullname)
                r.append(
                    '<tr><td><a href="%s">%s</a></td><td>%d</td><td>%s</td></tr>\n'
                    % (urllib.parse.quote(linkname, errors='surrogatepass'),
                       html.escape(displayname, quote=False), fs[6],
                       self.date_time_string(fs.st_mtime)))
            except FileNotFoundError:
                r.append(
                    '<tr><td><a href="%s">%s</a></td><td>UNKNOW</td><td>UNKNOW</td></tr>\n'
                    % (urllib.parse.quote(linkname, errors='surrogatepass'),
                       html.escape(displayname, quote=False)))

        r.append('</table>\n<hr>\n</body>\n</html>\n')
        encoded = '\n'.join(r).encode(enc, 'surrogateescape')
        f = io.BytesIO()
        f.write(encoded)
        f.seek(0)
        self.send_response(200)
        self.send_header("Content-type", "text/html; charset=%s" % enc)
        self.send_header("Content-Length", str(len(encoded)))
        self.end_headers()
        return f

    def log_message(self, format, *args):
        logging.info("%s - %s" % (self.client_address[0], format % args))


class ThreadingHTTPServer(socketserver.ThreadingMixIn, http.server.HTTPServer):
    daemon_threads = True


class Server(server.Server):
    """Rsync protocol."""
    def __init__(self, *args):
        """Constructor.

        :param  data_path:  Path of data directory, :class:`str` object.
        :param  distros:    Distros use this protocol, :class:`list` object.
        """
        super().__init__(*args)
        self.__cond = threading.Condition()

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

    def start(self):
        """Start service.
        """
        self.__config = config.config.server_protocol_cfg(self.name())
        requestHandler = type("RequestHandler", (RequestHandlerTemplate, ), {
            "DATA_DIRECTORY": self._data_path,
            "DISTROS": copy.copy(self._distros)
        })
        self.__server = ThreadingHTTPServer(
            (self.__config["address"], self.__config["port"]), requestHandler)
        self.start_work()

    def stop(self):
        """Stop service.
        """
        self.__server.shutdown()
        self.join()

    def work(self):
        self.__server.serve_forever()
