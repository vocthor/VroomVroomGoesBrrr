#!/usr/bin/env python3

"""
Run ./trackmania <action> <ports>

<action> can be 'down', 'up'
<ports> is numbers concatenated referencing the server to affect

ex : './trackmania.py up 12' -> starts cup1 and cup2
"""


import subprocess
import sys

NUMBER_ARGS = 2


def upServer(id):
    """
    It runs a docker-compose command in a specific directory

    :param id: the id of the server
    """
    name = "tm_server_" + id
    path = "./compose/cup" + id + "/"
    p = subprocess.Popen(["sudo", "docker-compose", "-p", name,
                          "-f", "docker-compose.yaml", "up", "-d"], cwd=path)
    print(p.communicate())


def downServer(id):
    """
    It takes an id as a parameter, and then it runs a docker-compose command to bring down the server
    with that id

    :param id: the id of the server
    """
    name = "tm_server_" + id
    path = "./compose/cup" + id + "/"
    p = subprocess.Popen(["sudo", "docker-compose", "-p", name,
                          "-f", "docker-compose.yaml", "down", "-v"], cwd=path)
    print(p.communicate())


def main(args):
    """
    It starts or closes the servers in the list of servers passed as an argument

    :param args: the list of arguments passed to the script
    """
    if (len(args) > 1+NUMBER_ARGS or len(args) < 1+NUMBER_ARGS):
        exit(1)
    listServer = args[2]
    if (args[1] == "up"):
        for i in range(len(listServer)):
            upServer(listServer[i])
    elif (args[1] == "down"):
        for i in range(len(listServer)):
            downServer(listServer[i])


if __name__ == "__main__":
    main(sys.argv)
