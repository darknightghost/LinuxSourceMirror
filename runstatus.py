#! /usr/bin/env python3
# -*- coding: utf-8 -*-

run = True
exit_code = 0


def exit(exit_code):
    global run
    global exit_code
    run = False
    exit_code = exit_code
