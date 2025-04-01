#include <signal.h>
#include <errno.h>
#include <mocklibc.h>

int sigprocmask(int how, const sigset_t *restrict set, sigset_t *restrict old)
{
	NOIMPL_STR("sigprocmask\n")
	// int r = pthread_sigmask(how, set, old);
	// if (!r) return r;
	// errno = r;
	// return -1;
}
