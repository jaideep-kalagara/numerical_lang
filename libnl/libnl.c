#define SYSCALL_WRITE 1
#define SYSCALL_EXIT 60

static inline long syscall3(long number, long arg1, long arg2, long arg3)
{
    long ret;

    __asm__ volatile(
        "syscall"
        : "=a"(ret)
        : "a"(number), "D"(arg1), "S"(arg2), "d"(arg3)
        : "rcx", "r11", "memory");

    return ret;
}

static inline void syscall1_noreturn(long number, long arg1)
{
    __asm__ volatile(
        "syscall"
        :
        : "a"(number), "D"(arg1)
        : "rcx", "r11", "memory");
    __builtin_unreachable();
}

void nl_write(const char *str)
{
    syscall3(SYSCALL_WRITE, 1, (long)str, sizeof(str));
}

void nl_exit(int error_code)
{
    syscall1_noreturn(SYSCALL_EXIT, error_code);
}