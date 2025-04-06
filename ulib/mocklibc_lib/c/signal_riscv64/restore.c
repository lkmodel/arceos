#include <features.h>
#include <mocklibc.h>

/* These functions will not work, but suffice for targets where the
 * kernel sigaction structure does not actually use sa_restorer. */

hidden void __restore(){NOIMPL_STR("__restore\n")}

hidden void __restore_rt()
{
    NOIMPL_STR("__restore_rt\n")
}
