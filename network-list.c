#include <sys/types.h>
#include <ifaddrs.h>
#include <sys/socket.h>
#include <netdb.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include "network-list.h"
#include "utils.h"

network_t *new_network(const char *name, const char *ip)
{
    network_t *out = malloc(sizeof(network_t));
    out->name = copystr(name);
    out->ip = copystr(ip);
    return out;
}

network_list_t new_network_list()
{
    network_list_t out = {0};

    struct ifaddrs *ifaddr;
    int family, s;
    char host[NI_MAXHOST];

    if (getifaddrs(&ifaddr) == -1)
    {
        fprintf(stderr, "getifaddrs failed\n");
    }

    for (struct ifaddrs *ifa = ifaddr; ifa != NULL; ifa = ifa->ifa_next)
    {
        if (ifa->ifa_addr == NULL || ifa->ifa_addr->sa_family != AF_INET)
        {
            continue;
        }

        int errcode = getnameinfo(ifa->ifa_addr, sizeof(struct sockaddr_in),
                                  host, NI_MAXHOST, NULL, 0, NI_NUMERICHOST);
        if (errcode != 0)
        {
            fprintf(stderr, "Failed to get IP of %s: %s\n", ifa->ifa_name, gai_strerror(errcode));
        }
        out.list[out.size++] = new_network(ifa->ifa_name, host);
    }

    freeifaddrs(ifaddr);

    return out;
}
